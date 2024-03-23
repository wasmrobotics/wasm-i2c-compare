use std::collections::HashMap;
use std::env;
use std::fs;

use anyhow::Result;
use wasmtime::{component, Config, Engine};
use wasmtime_wasi::command;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

struct MyState {
    wasi: WasiCtx,
    table: component::ResourceTable,
    host: sample_imu::ImuHC,
}
impl wasmtime_wasi::WasiView for MyState {
    fn table(&mut self) -> &mut component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> Result<()> {
    let _num_loops = if let Some(num_loops) = std::env::args().nth(1) {
        num_loops.parse::<u32>()?
    } else {
        1000
    };

    let mut env = HashMap::new();

    match env::var_os("PRINT_IMU") {
        Some(_) => {
            println!("PRINT_IMU");
            env.insert("PRINT_IMU", "true");
        }
        None => println!("No PRINT_IMU"),
    }
    let envs = env.into_iter().collect::<Vec<(&str, &str)>>();
    // env.push("PRINT_IMU");
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    // Create the engine and the linker.
    let mut config = Config::default();
    config.wasm_component_model(true);
    #[cfg(feature = "profile")]
    config.profiler(wasmtime::ProfilingStrategy::PerfMap);

    let engine = Engine::new(&config)?;
    let mut linker = component::Linker::new(&engine);
    sample_imu::wasm_robotics::robotics::imus::add_to_linker(
        &mut linker,
        |state: &mut MyState| &mut state.host,
    )?;
    command::sync::add_to_linker(&mut linker)?;

    // Read the guest component file.
    let path = "./imu-opmode.wasm";
    let component_bytes = fs::read(path).expect("failed to read input file");
    let component = component::Component::new(&engine, component_bytes)?;
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .envs(&envs)
        .build();
    // Create the state that will be stored in the store, and link it in.
    let my_state = MyState {
        wasi: wasi_ctx,
        table: component::ResourceTable::new(),
        host: sample_imu::ImuHC {
            imus: component::ResourceTable::new(),
        },
    };

    let mut store = wasmtime::Store::new(&engine, my_state);
    let instance = linker.instantiate(&mut store, &component)?;
    // call it!
    let func = {
        let mut exports = instance.exports(&mut store);

        let mut instance = exports
            .instance("wasi:cli/run@0.2.0")
            .expect("missing the expected 'wasi:cli/run@0.2.0' instance");
        instance.typed_func::<(), (Result<(), ()>,)>("run")?
    };

    let _ = func.call(&mut store, ()).expect("main call failed");
    func.post_return(&mut store).expect("post_return failed");
    Ok(())
}
