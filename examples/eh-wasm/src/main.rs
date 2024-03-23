#[allow(warnings)]
mod bindings;
use std::hint::black_box;

// use bno055::{BNO055OperationMode, Bno055};
use eh_wasm::imu_init;
// use embedded_hal::i2c::I2c;
use tracing::debug;
// use wasm_hal::{delay, i2c};
// use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
// fn write_calibration(calibration_data: CalibrationData, output_fname: &str) -> Result<(), String> {
//     let serialized = calibration::serialize(&calibration_data);
//     let mut output_file = fs::File::create(output_fname)
//         .map_err(|err| format!("error opening output {}: {}", output_fname, err))?;
//     output_file
//         .write_all(&serialized)
//         .map_err(|err| format!("write error: {}", err))
// }

// fn read_calibration(input_fname: &str) -> Result<CalibrationData> {
//     let mut input_file = fs::File::open(input_fname).map_err(|err| {
//         anyhow::Error::msg(format!("error opening input {}: {}", input_fname, err))
//     })?;
//     let mut contents = [0; 22];
//     input_file
//         .read(&mut contents)
//         .map_err(|err| anyhow::Error::msg(format!("read error: {}", err)))?;
//     Ok(calibration::deserialize(&contents))
// }

fn main() {
    let num_loops = if let Some(num_loops) = std::env::args().nth(1) {
        num_loops.parse::<u32>().expect("expected integer (u32)")
    } else {
        1000
    };
    let mut imu = imu_init().expect("init imu");
    let status = imu.get_calibration_status().unwrap();
    debug!("The IMU's calibration status is: {:?}", status);
    // TODO: add read calibration
    // Wait for device to auto-calibrate.
    // Please perform steps necessary for auto-calibration to kick in.
    // Required steps are described in Datasheet section 3.11
    // Page 51, https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bno055-ds000.pdf (As of 2021-07-02)
    // debug!("- About to begin BNO055 IMU calibration...");
    // while !imu.is_fully_calibrated().unwrap() {
    //     status = imu.get_calibration_status().unwrap();
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     debug!("Calibration status: {:?}", status);
    // }
    //  if let Ok(calibration_data) = read_calibration("imu-calibration.bin") {
    //         hw.imu
    //             .set_calibration(calibration_data)
    //             .expect("set imu calibration")
    //     } else {
    //         match hw.imu.calibrate() {
    //             Ok(data) => write_calibration(data, "imu-calibration.bin")
    //                 .map_err(|err| anyhow::Error::msg(format!("read error: {}", err)))?,
    //             Err(e) => {
    //                 edebug!("unable to write calibration_data {:?}", e);
    //             }
    //         }
    //     }
    // let calib = imu.calibration_profile(&mut delay).unwrap();

    // imu.set_calibration_profile(calib, &mut delay).unwrap();
    // debug!("       - Calibration complete!");

    // // These are sensor fusion reading using the mint crate that the state will be read into
    // let mut quaternion: Quaternion<f32>; // = Quaternion::<f32>::from([0.0, 0.0, 0.0, 0.0]);
    let start = std::time::Instant::now();
    for _ in 0..num_loops {
        black_box(imu.quaternion().expect("Quaternion"));
    }
    let elapsed = start.elapsed();
    println!("Elapsed time: {elapsed:?} (loops: {num_loops})");
}
