use anyhow::Result;
use bno055::{mint::Quaternion, BNO055OperationMode, Bno055};
use linux_embedded_hal::{Delay, I2CError, I2cdev};

pub fn imu_init() -> Result<Bno055<I2cdev>, Box<dyn std::error::Error>> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let mut imu = Bno055::new(i2c).with_alternative_address();
    let mut delay = Delay {};
    imu.init(&mut delay)
        .expect("An error occurred while building the IMU");

    imu.set_mode(BNO055OperationMode::NDOF, &mut delay)
        .expect("An error occurred while setting the IMU mode");

    Ok(imu)
}

pub fn imu_loop(imu: &mut Bno055<I2cdev>) -> Result<Quaternion<f32>, bno055::Error<I2CError>> {
    let quat = imu.quaternion()?;

    Ok(quat)
}
