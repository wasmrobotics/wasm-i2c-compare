# wasm-hal

A demonstration project exposing a WASM component based HAL to web assembly components

## Purpose

Our goal is to understand the approach to accessing sensor data via i2c in wasmspace.  We've experimented with a few approaches and this repo contains the results of those efforts.

We've taken the same functionality (reading a Quaternion value from an IMU) and implemented it in three ways:

In each of the applications we perform the following steps (albeit differently)

1. Initalize the IMU
  a. Acquire an i2c connection
  b. initialize the BNO055
  c. set the BNO055 into 9-DOF _Fusion_ mode
2. Capture the starting time
3. Loop 1000 times
  a. Read a Quaternion
4. Report elapsed time

- Native (`imu_native`) - a standalone binary utilizing the [BNO055 Crate](https://crates.io/crates/bno055)
- Driver in native space with wasm component caller (`imu_wasm_hc`)
- i2c driver in native space with wasm driver and caller (`imu_wasm_i2c`)

# Details

The `imu_native` code exists to provide a baseline for functionality and performance.  

The `imu_wasm_hc` demonstrates building a `HostComponent` that is added to the runtime linker and made available via a WIT based interface to the driver's functionality.

The `imu_wasm_i2c` demonstates building both a generalized HAL defined in WIT for i2c operations as well as a component based implementation of the [embedded_hal](https://crates.io/crates/embedded-hal) traits such that a publicly available driver (the BNO055) can we compiled and executed in wasmspace.  We believe that in general `#[no_std]` drivers that supported embedded_hal will be supportable with this mechanism.

The `imu_wasm_i2c` approach is modeled/derived from the [hello-embeded](https://github.com/sunfishcode/hello-embedded/tree/main) experiments. 

The focus on i2c rather than digital I/O is looking specifically at performance deltas and implementation approaches to the embedded_hal traits.  See [transaction](./crates/wasm_hal/src/i2c.rs#44) and [transaction-host](./crates/wasm_hal_host/src/lib.rs#97) for the specific areas where we're doing additional allocations to support the (current) limitations of WIT/HostComponent boundaries and support the semantics of the zero-abstraction api of embedded_hal [Read in particular](https://docs.rs/embedded-hal/latest/embedded_hal/i2c/enum.Operation.html#variant.Read) which uses a slice to eliminate return value allocations.

## Memory Allocations

Given the alternate approaches employed to collect the sensor data (and in particular the differences between IMU as Host Component vs HAL as Host Component) allocations should be the primary delta between those models.
