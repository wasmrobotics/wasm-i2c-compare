# IMU

## Hardware

## Running and configuration

The BNO055 needs to be calibrated.  

in this opmode we try to read (and then set) the calibration from a file named `imu-calibration.bin`

If the file is not found, we'll put the IMU into calibration mode and then write the calibration 

See [BNO055 Datasheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bno055-ds000.pdf) for the calibration incantations

See Section 3.11 particularly.

It is not clear to me if we can simply reuse the profile/calibration always.  In any case the methods are here to control that from wasmspace.

Because we're interacting with a file we need to set the appropriate sandbox wormhole to read and write the filesystem.

the `--dir .` directive below opens access for read/write/all to this runtime invocation.

```sh
./bepr run --dir . --imu imu-opmode.wasm
```

see examples/file-io-opmode for a static test app that just does reading and writing without actually interacting with the imu

also note `--imu` enables the IMU Host Component

