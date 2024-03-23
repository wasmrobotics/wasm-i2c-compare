use embedded_hal::i2c::{self as eh_i2c, ErrorKind, ErrorType, I2c};
use linux_embedded_hal::I2cdev;

pub struct I2CSpy {
    pub(crate) inner: I2cdev,
}

impl I2CSpy {
    pub fn new(ident: &str) -> Self {
        let inner = I2cdev::new(ident).expect("couldn't create I2C in I2CSpy");
        I2CSpy { inner }
    }
}

impl ErrorType for I2CSpy {
    type Error = ErrorKind;
}

impl I2c for I2CSpy {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [eh_i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations.iter_mut() {
            println!("I2CSpy: transaction {address:02X}");
            match op {
                eh_i2c::Operation::Read(buf) => {
                    println!("\tI2CSpy:(pre) read op {:02x?}", buf.len());
                }
                eh_i2c::Operation::Write(buf) => {
                    println!("\tI2CSpy:(pre) write op {buf:02x?}");
                }
            }
        }
        self.inner
            .transaction(address, operations)
            .map(|_| {
                for op in operations.iter_mut() {
                    match op {
                        eh_i2c::Operation::Read(buf) => {
                            println!("\tI2CSpy: read op {:02x?}", buf);
                        }
                        eh_i2c::Operation::Write(_buf) => {
                            // println!("\tI2CSpy: write op {:02x?}", buf.len());
                        }
                    }
                }
            })
            .map_err(|_| ErrorKind::Other) // TODO
    }
}
