use bno055::{mint::Quaternion, Bno055};
use criterion::{criterion_group, criterion_main, Criterion};
use linux_embedded_hal::{I2CError, I2cdev};
// use imu_native::*;

fn read_imu(imu: &mut Bno055<I2cdev>) -> Result<Quaternion<f32>, bno055::Error<I2CError>> {
    let quat = imu.quaternion()?;
    Ok(quat)
}

fn criterion_benchmark_native_imu(c: &mut Criterion) {
    let mut imu = imu_native::imu_init().unwrap();
    c.bench_function("native_imu", |b| b.iter(|| read_imu(&mut imu)));
}

criterion_group!(benches, criterion_benchmark_native_imu);
criterion_main!(benches);
