use embedded_hal::i2c::I2c as _;
use embedded_hal::i2c::{self as eh_i2c};
#[cfg(target_os = "linux")]
use linux_embedded_hal::I2cdev;
use tracing::{instrument, trace};
use wasmtime::component::{self};

use wasm_robotics::robotics::{
    delay,
    i2c::{self, ErrorCode},
};

component::bindgen!({
    path: "../../wit",  // Save yourself - this is a path to the root of the wit directory, not a specific file.
    world: "bp-hal",
    with: {
        "wasm-robotics:robotics/delay/delay": Delay,
        "wasm-robotics:robotics/i2c/i2c": I2C,
    }

});

pub struct Delay {}
#[derive(Default)]
pub struct DelayHC {
    pub delays: component::ResourceTable,
}

impl DelayHC {}
impl delay::HostDelay for DelayHC {
    #[instrument(level = "trace", skip(self))]
    fn new(&mut self) -> wasmtime::Result<wasmtime::component::Resource<delay::Delay>> {
        Ok(self.delays.push(Delay {}).expect("new delay resource"))
    }
    #[instrument(level = "trace", skip(self))]
    fn delay_ns(
        &mut self,
        delay_resource: wasmtime::component::Resource<delay::Delay>,
        ns: u32,
    ) -> wasmtime::Result<()> {
        trace!("delay NS {ns}");
        let _self_ = self.delays.get_mut(&delay_resource);
        std::thread::sleep(std::time::Duration::from_nanos(ns.into()));

        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    fn drop(
        &mut self,
        delay_resource: wasmtime::component::Resource<delay::Delay>,
    ) -> wasmtime::Result<()> {
        trace!("delay drop");
        Ok(self
            .delays
            .delete(delay_resource)
            .map(|_| ())
            .map_err(|_| ErrorCode::Other)?)
    }
}

impl delay::Host for DelayHC {}

pub struct I2C {
    inner: I2cdev,
}

#[derive(Default)]
pub struct I2cHC {
    pub table: component::ResourceTable,
}

impl i2c::HostI2c for I2cHC {
    fn new(
        &mut self,
        identifier: String,
    ) -> wasmtime::Result<wasmtime::component::Resource<i2c::I2c>> {
        let dev = I2cdev::new(identifier).expect("native i2cdriver acquisition");
        // // let dev = I2CSpy::new("/dev/i2c-1"); //.unwrap();
        Ok(self
            .table
            .push(I2C { inner: dev })
            .map_err(|_| ErrorCode::Other)?)
    }
    #[instrument(level = "trace", skip(self))]
    fn transaction(
        &mut self,
        i2c_resource: wasmtime::component::Resource<i2c::I2c>,
        address: i2c::Address,
        operations: Vec<i2c::Operation>,
    ) -> wasmtime::Result<Result<Vec<Vec<u8>>, ErrorCode>> {
        let i2c = self
            .table
            .get_mut(&i2c_resource)
            .expect("i2c resource available");

        let mut operation_results: Vec<Vec<u8>> = Vec::new();
        for op in operations.iter() {
            trace!("transaction op {:02X}", address);
            match op {
                i2c::Operation::Read(len) => {
                    trace!(
                        "read op {:02X} preallocating len {}",
                        address,
                        *len as usize
                    );
                    operation_results.push(vec![0; *len as usize]);
                }
                i2c::Operation::Write(_data) => {
                    operation_results.push(Vec::with_capacity(0));
                }
            }
        }
        trace!(
            "operation results before operation: {:?}",
            operation_results
        );
        let mut our_ops = operations
            .iter()
            .zip(operation_results.iter_mut())
            .map(|(op, ores)| match op {
                i2c::Operation::Read(_len) => eh_i2c::Operation::Read(ores),
                i2c::Operation::Write(data) => eh_i2c::Operation::Write(data),
            })
            .collect::<Vec<eh_i2c::Operation>>();

        let _ = i2c.inner.transaction(address, &mut our_ops);
        trace!("operation results after operation: {:?}", operation_results);

        trace!("transaction op {:02X} {:?}", address, operations);
        wasmtime::Result::Ok(Ok(operation_results))
    }

    #[instrument(level = "trace", skip(self))]
    fn read(
        &mut self,
        i2c_resource: wasmtime::component::Resource<i2c::I2c>,
        address: i2c::Address,
        len: u64,
    ) -> wasmtime::Result<Result<Vec<u8>, ErrorCode>> {
        let i2c = self
            .table
            .get_mut(&i2c_resource)
            .expect("i2c resource available");
        trace!("read op address 0x{:02X} {len}", address);
        let effective_len = len as u8;
        let mut buffer: Vec<u8> = Vec::with_capacity(effective_len.into());

        Ok(i2c
            .inner
            .read(address, &mut buffer)
            .map_err(|_oe| ErrorCode::Other) // TODO: cleanup
            .map(|_| {
                trace!("read {:02X?}", &buffer);

                buffer
            }))
    }

    #[instrument(level = "trace", skip(self))]
    fn write(
        &mut self,
        i2c_resource: wasmtime::component::Resource<i2c::I2c>,
        address: i2c::Address,
        data: Vec<u8>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let i2c = self
            .table
            .get_mut(&i2c_resource)
            .expect("i2c resource available");
        trace!(
            "write op len:{} address:0x{:02X} data:{:?}",
            data.len(),
            address,
            data
        );
        trace!("writing {:02X?}", &data);

        Ok(i2c
            .inner
            .write(address, &data)
            .map_err(|_oe| ErrorCode::Other)) // TODO: Errors
    }

    #[instrument(level = "trace", skip(self), fields(write_len=write.len(), read_len=read_len))]
    fn write_read(
        &mut self,
        i2c_resource: wasmtime::component::Resource<i2c::I2c>,
        address: i2c::Address,
        write: Vec<u8>,
        read_len: u64,
    ) -> wasmtime::Result<Result<Vec<u8>, ErrorCode>> {
        let i2c = self
            .table
            .get_mut(&i2c_resource)
            .expect("i2c resource available");
        trace!(
            "write_read write address: 0x{:02X} len {} read len  {}",
            address,
            write.len(),
            read_len
        );
        let effective_len = read_len as u8;
        let mut buffer: Vec<u8> = Vec::with_capacity(effective_len.into());
        trace!("write_read (write buf) {:02X?}", &write);
        Ok(i2c
            .inner
            .write_read(address, &write, &mut buffer)
            .map_err(|_oe| ErrorCode::Other) // TODO cleanup errors
            .map(|_| {
                trace!("read result{:02X?}", &buffer);

                buffer
            }))
    }

    fn drop(
        &mut self,
        i2c_resource: wasmtime::component::Resource<i2c::I2c>,
    ) -> wasmtime::Result<()> {
        let _ = self.table.delete(i2c_resource);
        Ok(())
    }
}
impl i2c::Host for I2cHC {
    fn i2c_named(
        &mut self,
        identifier: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<i2c::I2c>, ErrorCode>> {
        let dev = I2cdev::new(identifier).expect("native i2cdriver acquisition");

        Ok(Ok(self
            .table
            .push(I2C { inner: dev })
            .map_err(|_| ErrorCode::Other)?))
    }
}
