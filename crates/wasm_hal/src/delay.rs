use std::thread;
use std::time::Duration;

use embedded_hal::delay::DelayNs;

use crate::bindings::wasm_robotics::robotics::delay::Delay as HAL_Delay;

pub struct Delay {
    _inner: HAL_Delay,
}

impl Delay {
    pub fn new() -> Self {
        let hal_delay = HAL_Delay::new();
        Delay { _inner: hal_delay }
    }
}

impl Default for Delay {
    fn default() -> Self {
        Self::new()
    }
}

impl DelayNs for Delay {
    fn delay_ns(&mut self, n: u32) {
        println!("DelayNs::delay_ns({})", n);
        thread::sleep(Duration::from_nanos(n.into()));
    }

    fn delay_us(&mut self, n: u32) {
        thread::sleep(Duration::from_micros(n.into()));
    }

    fn delay_ms(&mut self, n: u32) {
        thread::sleep(Duration::from_millis(n.into()));
    }
}
