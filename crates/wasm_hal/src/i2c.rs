use embedded_hal::i2c::{self as eh_i2c, ErrorKind, ErrorType, I2c};

use crate::bindings::wasm_robotics::robotics::{i2c, i2c::I2c as HAL_i2c};

// const MODULE: &str = "WI2C";
pub struct WI2C {
    pub(crate) inner: HAL_i2c,
}

impl WI2C {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(_ident: &str) -> impl I2c<Error = ErrorKind> {
        // e// println!("{MODULE} new WI2C called");
        let inner = i2c::i2c_named(_ident).expect("couldn't create I2C in WI2C");
        WI2C { inner }
    }
}

impl ErrorType for WI2C {
    type Error = ErrorKind;
}

impl I2c for WI2C {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [eh_i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        // TODO: should be a `From` over the operations slice
        let our_ops = operations
            .iter()
            .map(|op| op.into())
            .collect::<Vec<i2c::Operation>>();
        let result = self.inner.transaction(address.into(), &our_ops);
        // println!("{MODULE}: transaction result {:?}", result);
        match result {
            Ok(operation_results) => {
                let r_iter = operation_results.iter().zip(operations.iter_mut());
                for (r, op) in r_iter {
                    match op {
                        eh_i2c::Operation::Read(buf) => {
                            buf.copy_from_slice(r);
                        }
                        eh_i2c::Operation::Write(_) => {}
                    }
                }
            }
            Err(_) => todo!(),
        };
        Ok(())
    }
    // We only _need to implement transaction - everything else will be implemented in terms of calls to transaction

    // fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
    //     // e// println!(
    //         "{MODULE}read for {} bytes to address 0x{:02x}",
    //         read.len(),
    //         address
    //     );
    //     let result = self
    //         .inner
    //         .read(address.into(), read.len().try_into().unwrap())
    //         .expect("Read to succeed");
    //     // println!("Bytes returned = {:?}", result);
    //     read.copy_from_slice(&result); // uggh, no way to avoid this for now
    //                                    // *read = result.as_slice();
    //     Ok(())
    //     // self.transaction(address, &mut [eh_i2c::Operation::Read(read)])
    // }

    // fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
    //     // println!(
    //         "{MODULE} writing for {} bytes to address 0x{:02x}",
    //         write.len(),
    //         address
    //     );
    //     // let result =
    //     Ok(self
    //         .inner
    //         .write(address.into(), write)
    //         .expect("{MODULE} write should succeed"))
    //     // // e// println!("Bytes returned = {:?}", result);
    //     // let slice = result.
    //     // *read =
    //     // Ok(())
    //     // self.transaction(address, &mut [eh_i2c::Operation::Write(write)])
    // }

    // fn write_read(
    //     &mut self,
    //     address: u8,
    //     write: &[u8],
    //     read: &mut [u8],
    // ) -> Result<(), Self::Error> {
    //     // println!(
    //         "{MODULE} write_read for {} write bytes, read bytes {} to address 0x{:02x}",
    //         write.len(),
    //         read.len(),
    //         address
    //     );
    //     let result = self
    //         .inner
    //         .write_read(address.into(), write, read.len().try_into().unwrap())
    //         .expect("write_read");
    //     // println!("{MODULE} write_read Bytes returned = {:?}", &result);
    //     read.copy_from_slice(&result);
    //     Ok(())
    // }
}

impl From<eh_i2c::Operation<'_>> for i2c::Operation {
    fn from(value: eh_i2c::Operation) -> Self {
        match value {
            eh_i2c::Operation::Read(buf) => i2c::Operation::Read(buf.len().try_into().unwrap()),

            eh_i2c::Operation::Write(buf) => i2c::Operation::Write(buf.to_vec()),
        }
    }
}

impl<'a> From<&'a embedded_hal::i2c::Operation<'_>> for i2c::Operation {
    fn from(value: &'a embedded_hal::i2c::Operation) -> Self {
        match value {
            eh_i2c::Operation::Read(buf) => i2c::Operation::Read(buf.len().try_into().unwrap()),

            eh_i2c::Operation::Write(buf) => i2c::Operation::Write(buf.to_vec()),
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_transform_operation() {
        let mut buf: [u8; 1] = [0; 1];
        let x = eh_i2c::Operation::Read(&mut buf);
        let y: i2c::Operation = x.into();
        let read_op_found = match y {
            i2c::Operation::Read(_) => true,
            i2c::Operation::Write(_) => panic!("Should be a read"),
        };
        assert!(read_op_found, "Expected one read operation")
    }

    #[test]
    fn test_result_handling() {
        // let mut buf: [u8; 1] = [0; 1];
        let mut r_buf1: [u8; 2] = [0; 2];
        let mut r_buf2: [u8; 2] = [0xff; 2];
        // given a list of host operations results
        // transform each, applying the contained results to the original operations passed in
        // this is really just handling the Read operation where we're receiving a &mut [u8] but getting a Vec<u8> back from the host
        assert_eq!(&r_buf1[..], &[0, 0]);
        assert_eq!(&r_buf2[..], &[0xff, 0xff]); // Fix: Borrow `r_buf2` as immutable using slicing
        let mut operations = vec![
            eh_i2c::Operation::Read(&mut r_buf1),
            eh_i2c::Operation::Write(&[0x01]),
            eh_i2c::Operation::Read(&mut r_buf2),
        ];

        let host_results = vec![vec![0x01, 0x02], vec![], vec![0x03, 0x04]];
        let r_iter = host_results.iter().zip(operations.iter_mut());
        for (r, op) in r_iter {
            // println!("{MODULE}: decoding transaction op {:02x?}  {:?}", r, op);

            match op {
                eh_i2c::Operation::Read(buf) => {
                    // println!("{MODULE}: read op {:02x?}", r);
                    buf.copy_from_slice(r);
                }
                eh_i2c::Operation::Write(_) => {}
            }
        }

        assert_eq!(r_buf1, [0x01, 0x02]);
        assert_eq!(r_buf2, [0x03, 0x04]);
    }
}
