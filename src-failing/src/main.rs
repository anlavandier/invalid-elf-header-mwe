use wasmtime::{component::Component, *};
use wasmtime::component::bindgen;

fn main () {
    if let Err(e) = run_wasmtime() {
        println!("{:?}", defmt::Debug2Format(&e))
    }
}


fn run_wasmtime() -> wasmtime::Result<()> {
    let mut config = Config::default();

    config.target("pulley32")?;
    let engine = Engine::new(&config)?;
    let component_bytes  = include_bytes!("path/to/add_wasm32-wasip1.cwasm");

    let component = match unsafe { Component::deserialize_raw(&engine, component_bytes.as_slice().into()) } {
        Ok(comp) => comp,
        Err(error) => {
            println!("failed to deserialize pre-compiled module: {:?}", defmt::Debug2Format(&error));
            return Ok(());
        }
    };
    println!("Deserialized Component");
}




