use std::fmt;

use anyhow::Result;
use bno055::{
    mint::Quaternion as mint_Quaternion, AxisRemap as BNO055AxisRemap, BNO055AxisConfig,
    BNO055AxisSign, BNO055Calibration, BNO055OperationMode, BNO055PowerMode, Bno055,
};
use linux_embedded_hal::{Delay, I2cdev};
use tracing::{debug, info};
use wasmtime::component::{self, bindgen};

use wasm_robotics::robotics::{
    imus::{
        self, AccessError, AxisConfig, AxisRemap, AxisSigns, CalibrationData, Imu as HostImu,
        PowerMode, Quaternion,
    },
    types::AccessError::{HardwareAccessError, NamedResourceNotFound},
};

bindgen!({
    path: "../../wit",  // Save yourself - this is a path to the root of the wit directory, not a specific file.
    world: "imu-client",
    with: {
        "wasm-robotics:robotics/imus/imu": Imu,
    }

});

pub struct Imu {
    bno055: bno055::Bno055<I2cdev>,
    name: String,
}

pub fn build_imu(name: &str) -> Result<Imu> {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut delay = Delay {};
    let mut imu = Bno055::new(dev).with_alternative_address();
    imu.init(&mut delay)
        .expect("An error occurred while building the IMU");
    imu.set_mode(BNO055OperationMode::NDOF, &mut delay)
        .expect("An error occurred while setting the IMU mode");

    Ok(Imu {
        name: name.to_string(),
        bno055: imu,
    })
}

impl std::fmt::Debug for Imu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " imu: {:?}", self.name)
    }
}

impl imus::Host for ImuHC {
    fn imu_named(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Imu>, AccessError>> {
        Ok(self
            .imus
            .push(build_imu(&name).map_err(|e| {
                NamedResourceNotFound(format!("Failure creating imu named {name} {:?}", e))
            })?)
            .map_err(|e| {
                NamedResourceNotFound(format!("Failure creating imu named {name} {:?}", e))
            }))
    }
}

// pub struct Imu(bno055::Bno055<I2cdev>);
#[derive(Debug, Default)]
pub struct ImuHC {
    pub imus: component::ResourceTable,
}

impl imus::HostImu for ImuHC {
    fn calibrate(
        &mut self,
        imu_resource: component::Resource<imus::Imu>,
    ) -> std::result::Result<
        std::result::Result<CalibrationData, wasm_robotics::robotics::types::AccessError>,
        anyhow::Error,
    > {
        let mut delay = Delay {};
        let imu = self.imus.get_mut(&imu_resource).expect("imu not found");
        let mut status = imu.bno055.get_calibration_status().unwrap();
        debug!("- About to begin BNO055 IMU calibration...");
        while !imu.bno055.is_fully_calibrated().unwrap() {
            status = imu.bno055.get_calibration_status().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(1000));
            debug!("Calibration status: {:?}", status);
        }
        debug!("The IMU's calibration status is: {:?}", status);

        match imu.bno055.calibration_profile(&mut delay) {
            Ok(data) => Ok(Ok(data.into())),
            Err(_) => todo!(),
        }
    }

    fn set_calibration(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        data: CalibrationData,
    ) -> std::result::Result<
        std::result::Result<(), wasm_robotics::robotics::types::AccessError>,
        anyhow::Error,
    > {
        let mut delay = Delay {};

        match self
            .imus
            .get_mut(&imu_resource)
            .expect("imu not found")
            .bno055
            .set_calibration_profile(data.into(), &mut delay)
        {
            Ok(_) => Ok(Ok(())),
            Err(_) => todo!(),
        }
    }

    fn quaternion(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
    ) -> wasmtime::Result<
        std::result::Result<Quaternion, wasm_robotics::robotics::types::AccessError>,
    > {
        Ok(self
            .imus
            .get_mut(&imu_resource)
            .expect("imu not found")
            .bno055
            .quaternion()
            .map_err(|e| HardwareAccessError(format!("{:?}", e)))
            .map(|f| Ok(f.into()))?)
    }
    fn drop(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
    ) -> wasmtime::Result<()> {
        let _ = self.imus.delete(imu_resource);
        Ok(())
    }

    #[doc = " TODO: should this be part of the API which makes it very BNO055 specific? or more general and include this in the hardware map?"]
    fn set_external_crystal(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        enabled: bool,
    ) -> wasmtime::Result<std::result::Result<(), wasm_robotics::robotics::types::AccessError>>
    {
        let mut delay = Delay {};

        Ok(self
            .imus
            .get_mut(&imu_resource)
            .expect("imu not found")
            .bno055
            .set_external_crystal(enabled, &mut delay)
            .map_err(|e| HardwareAccessError(format!("{:?}", e)))
            .map(|_| Ok(()))?)
    }

    fn set_axis_remap(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        remap: AxisRemap,
    ) -> wasmtime::Result<Result<(), AccessError>> {
        Ok(self
            .imus
            .get_mut(&imu_resource)
            .expect("imu not found")
            .bno055
            .set_axis_remap(remap.into())
            .map_err(|e| HardwareAccessError(format!("{:?}", e)))
            .map(|_| Ok(()))?)
    }

    fn axis_remap(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
    ) -> wasmtime::Result<Result<AxisRemap, AccessError>> {
        Ok(self
            .imus
            .get_mut(&imu_resource)
            .expect("imu not found")
            .bno055
            .axis_remap()
            .map_err(|e| HardwareAccessError(format!("{:?}", e)))
            .map(|f: BNO055AxisRemap| Ok(f.into()))?)
    }

    fn set_axis_signs(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        signs: AxisSigns,
    ) -> wasmtime::Result<Result<(), AccessError>> {
        Ok(self
            .imus
            .get_mut(&imu_resource)
            .expect("imu not found")
            .bno055
            .set_axis_sign(signs.into())
            .map_err(|e| HardwareAccessError(format!("{:?}", e)))
            .map(|_| Ok(()))?)
    }

    fn axis_signs(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
    ) -> wasmtime::Result<Result<AxisSigns, AccessError>> {
        Ok(self
            .imus
            .get_mut(&imu_resource)
            .expect("imu not found")
            .bno055
            .axis_sign()
            .map_err(|e| HardwareAccessError(format!("{:?}", e)))
            .map(|f| Ok(f.into()))?)
    }

    fn set_power_mode(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        mode: PowerMode,
    ) -> wasmtime::Result<Result<(), AccessError>> {
        Ok(self
            .imus
            .get_mut(&imu_resource)
            .expect("imu not found")
            .bno055
            .set_power_mode(mode.into())
            .map_err(|e| HardwareAccessError(format!("{:?}", e)))
            .map(|_| Ok(()))?)
    }
}

impl From<mint_Quaternion<f32>> for Quaternion {
    fn from(val: mint_Quaternion<f32>) -> Self {
        Quaternion {
            x: val.v.x,
            y: val.v.y,
            z: val.v.z,
            w: val.s,
        }
    }
}

impl From<BNO055Calibration> for CalibrationData {
    fn from(val: BNO055Calibration) -> Self {
        CalibrationData {
            acc_offset_x_lsb: val.acc_offset_x_lsb,
            acc_offset_x_msb: val.acc_offset_x_msb,
            acc_offset_y_lsb: val.acc_offset_y_lsb,
            acc_offset_y_msb: val.acc_offset_y_msb,
            acc_offset_z_lsb: val.acc_offset_z_lsb,
            acc_offset_z_msb: val.acc_offset_z_msb,
            mag_offset_x_lsb: val.mag_offset_x_lsb,
            mag_offset_x_msb: val.mag_offset_x_msb,
            mag_offset_y_lsb: val.mag_offset_y_lsb,
            mag_offset_y_msb: val.mag_offset_y_msb,
            mag_offset_z_lsb: val.mag_offset_z_lsb,
            mag_offset_z_msb: val.mag_offset_z_msb,
            gyr_offset_x_lsb: val.gyr_offset_x_lsb,
            gyr_offset_x_msb: val.gyr_offset_x_msb,
            gyr_offset_y_lsb: val.gyr_offset_y_lsb,
            gyr_offset_y_msb: val.gyr_offset_y_msb,
            gyr_offset_z_lsb: val.gyr_offset_z_lsb,
            gyr_offset_z_msb: val.gyr_offset_z_msb,
            acc_radius_lsb: val.acc_radius_lsb,
            acc_radius_msb: val.acc_radius_msb,
            mag_radius_lsb: val.mag_radius_lsb,
            mag_radius_msb: val.mag_radius_msb,
        }
    }
}

impl From<CalibrationData> for BNO055Calibration {
    fn from(val: CalibrationData) -> Self {
        BNO055Calibration {
            acc_offset_x_lsb: val.acc_offset_x_lsb,
            acc_offset_x_msb: val.acc_offset_x_msb,
            acc_offset_y_lsb: val.acc_offset_y_lsb,
            acc_offset_y_msb: val.acc_offset_y_msb,
            acc_offset_z_lsb: val.acc_offset_z_lsb,
            acc_offset_z_msb: val.acc_offset_z_msb,
            mag_offset_x_lsb: val.mag_offset_x_lsb,
            mag_offset_x_msb: val.mag_offset_x_msb,
            mag_offset_y_lsb: val.mag_offset_y_lsb,
            mag_offset_y_msb: val.mag_offset_y_msb,
            mag_offset_z_lsb: val.mag_offset_z_lsb,
            mag_offset_z_msb: val.mag_offset_z_msb,
            gyr_offset_x_lsb: val.gyr_offset_x_lsb,
            gyr_offset_x_msb: val.gyr_offset_x_msb,
            gyr_offset_y_lsb: val.gyr_offset_y_lsb,
            gyr_offset_y_msb: val.gyr_offset_y_msb,
            gyr_offset_z_lsb: val.gyr_offset_z_lsb,
            gyr_offset_z_msb: val.gyr_offset_z_msb,
            acc_radius_lsb: val.acc_radius_lsb,
            acc_radius_msb: val.acc_radius_msb,
            mag_radius_lsb: val.mag_radius_lsb,
            mag_radius_msb: val.mag_radius_msb,
        }
    }
}

impl From<AxisSigns> for BNO055AxisSign {
    fn from(val: AxisSigns) -> Self {
        let mut value: BNO055AxisSign = BNO055AxisSign::empty();
        if val.x_negative {
            value |= BNO055AxisSign::X_NEGATIVE;
        }
        if val.y_negative {
            value |= BNO055AxisSign::Y_NEGATIVE;
        }
        if val.z_negative {
            value |= BNO055AxisSign::Z_NEGATIVE;
        }
        value
    }
}

impl From<BNO055AxisSign> for AxisSigns {
    fn from(val: BNO055AxisSign) -> Self {
        let value: u8 = val.bits();
        AxisSigns {
            x_negative: (value & BNO055AxisSign::X_NEGATIVE.bits()) != 0,
            y_negative: (value & BNO055AxisSign::Y_NEGATIVE.bits()) != 0,
            z_negative: (value & BNO055AxisSign::Z_NEGATIVE.bits()) != 0,
        }
    }
}

impl From<PowerMode> for BNO055PowerMode {
    fn from(val: PowerMode) -> Self {
        match val {
            PowerMode::Normal => BNO055PowerMode::NORMAL,
            PowerMode::LowPower => BNO055PowerMode::LOW_POWER,
            PowerMode::Suspend => BNO055PowerMode::SUSPEND,
        }
        // PowerMode {}
    }
}

impl From<AxisRemap> for BNO055AxisRemap {
    fn from(_val: AxisRemap) -> Self {
        info!("Incompletely implemented!!! ");
        BNO055AxisRemap::builder()
            .swap_x_with(BNO055AxisConfig::AXIS_AS_Y)
            .build()
            .expect("axis remap")
    }
}

impl From<BNO055AxisRemap> for AxisRemap {
    fn from(val: BNO055AxisRemap) -> Self {
        info!("Incompletely implemented!!! ");
        AxisRemap {
            x: val.x().into(),
            y: val.y().into(),
            z: val.z().into(),
        }
    }
}

impl From<BNO055AxisConfig> for AxisConfig {
    fn from(val: BNO055AxisConfig) -> Self {
        match val {
            BNO055AxisConfig::AXIS_AS_X => AxisConfig::AxisX,
            BNO055AxisConfig::AXIS_AS_Y => AxisConfig::AxisY,
            BNO055AxisConfig::AXIS_AS_Z => AxisConfig::AxisZ,
            _ => todo!(),
        }
    }
}
