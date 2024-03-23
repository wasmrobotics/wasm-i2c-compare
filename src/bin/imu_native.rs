use anyhow::Result;
use tracing::debug;
// use tracing::debug;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tracing::instrument]
fn main() -> Result<()> {
    let num_loops: usize = if let Some(num_loops) = std::env::args().nth(1) {
        num_loops.parse::<usize>()?
    } else {
        1000
    };
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let mut imu = imu_native::imu_init().expect("init imu");
    let mut timings = Vec::with_capacity(num_loops);
    for _ in 0..num_loops {
        let start = std::time::Instant::now();
        let q = imu.quaternion().expect("quaternion");
        let elapsed = start.elapsed();
        timings.push(elapsed);
        debug!("q: {:?} {elapsed:?}", q);
    }
    // let elapsed = elapsed.checked_sub(rhs)
    println!("Elapsed time: {timings:?} (loops: {num_loops})");
    Ok(())
}
