use bno055::{mint::Quaternion, Bno055};
use criterion::{criterion_group, criterion_main, Criterion};
use embedded_hal::i2c::I2c;

fn read_imu(imu: &mut Bno055<impl I2c>) -> Result<Quaternion<f32>, Box<dyn std::error::Error>> {
    let quat = imu.quaternion().expect("quat read failed");
    Ok(quat)
}

fn criterion_benchmark_native_imu(c: &mut Criterion) {
    let mut imu = eh_wasm::imu_init().unwrap();
    c.bench_function("native_imu", |b| b.iter(|| read_imu(&mut imu)));
}

criterion_group!(benches, criterion_benchmark_native_imu);
criterion_main!(benches);
