use anyhow::Result;
use bindings::{
    exports::wasm_robotics::robotics::run::Guest,
    wasm_robotics::robotics::imus::{self},
};

use crate::bindings::wasm_robotics::robotics::imus::Quaternion;
#[allow(dead_code)]
mod bindings;

struct Hardware {
    imu: imus::Imu,
}

// Do the setup things
fn init() -> Result<Hardware, imus::AccessError> {
    println!("Initializing");
    Ok(Hardware {
        imu: imus::imu_named("28")?,
    })
}

struct Component;

impl Guest for Component {
    fn run(loop_count: u32) {
        let hw = init().expect("IMU Initialization");

        for _ in 0..loop_count {
            hw.imu.quaternion().expect("Quaternion");
        }
    }
}

fn main() -> Result<()> {
    let num_loops = if let Some(num_loops) = std::env::args().nth(1) {
        num_loops.parse::<usize>().expect("expected integer (u32)")
    } else {
        1000
    };
    // let num_loops = 1000;
    let hw = init().expect("IMU Initialization");

    let print = match std::env::var_os("PRINT_IMU") {
        Some(_) => |q: Quaternion| println!("q: {:?}", q),
        None => |_| {},
    };

    let mut timings = Vec::with_capacity(num_loops);

    for _ in 0..num_loops {
        let start = std::time::Instant::now();
        let q = hw.imu.quaternion().expect("Quaternion");
        let elapsed = start.elapsed();
        timings.push(elapsed);
        print(q);
    }
    println!("Elapsed time: {timings:?} (loops: {num_loops})");
    Ok(())
}

bindings::export!(Component with_types_in bindings);
