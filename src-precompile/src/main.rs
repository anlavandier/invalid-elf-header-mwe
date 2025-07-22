//! Pre-compiling a Wasm program.

use wasmtime::{Config, Engine, Result};

fn main() -> Result<()> {
    let mut config = Config::new();
    // config.wasm_custom_page_sizes(true);
    if let Err(error) = config.target("pulley32") {
        eprintln!(
            "this Wasmtime was not built with the correct compiler backend \
             enabled: {error:?}",
        );
        return Ok(());
    }

    // Create an `Engine` with that configuration.
    let engine = match Engine::new(&config) {
        Ok(engine) => engine,
        Err(error) => {
            println!("Wasmtime build is incompatible with config: {error:?}");
            return Ok(());
        }
    };

    // Pre-compile a Wasm program.
    //
    // Note that passing the Wasm text format, like we are doing here, is only
    // supported when the `wat` cargo feature is enabled.
    let wasm = include_bytes!("../../add/target/wasm32-wasip1/release/add.wasm");

    let precompiled = engine.precompile_component(wasm)?;

    // Write the pre-compiled program to a file.
    //
    // Note that the `.cwasm` extension is conventional for these files, and is
    // what the Wasmtime CLI will use by default, for example.
    std::fs::write("add_wasm32-wasip1.cwasm", &precompiled)?;

    // And we are done -- now a different Wasmtime embedding can load and run
    // the pre-compiled Wasm program from that `add.cwasm` file!
    Ok(())
}