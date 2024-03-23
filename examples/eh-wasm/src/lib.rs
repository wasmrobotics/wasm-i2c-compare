#[allow(warnings)]
mod bindings;
use bno055::{BNO055OperationMode, Bno055};
use embedded_hal::i2c::I2c;
use tracing::{debug, trace};
use wasm_hal::{delay, i2c};

pub fn imu_init() -> Result<Bno055<impl I2c>, Box<dyn std::error::Error>> {
    let dev = i2c::WI2C::new("/dev/i2c-1");
    trace!("I2C device created");
    // let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut delay = delay::Delay::new();
    let mut imu = Bno055::new(dev).with_alternative_address();
    debug!("IMU created");
    imu.init(&mut delay)
        .expect("An error occurred while building the IMU");
    debug!("IMU initialized");
    imu.set_mode(BNO055OperationMode::NDOF, &mut delay)
        .expect("An error occurred while setting the IMU mode");
    debug!("imu set mode returned");
    Ok(imu)
}
