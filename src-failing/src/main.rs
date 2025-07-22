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

    println!("Deserializing Component");

    // Create a runtime `Module` from a Wasm program that was pre-compiled and
    // written to the `add.cwasm` file by `wasmtime/examples/pre_compile.rs`.
    //
    // **Warning:** Wasmtime does not (and in general cannot) fully validate
    // pre-compiled modules for safety -- only create `Module`s and `Component`s
    // from pre-compiled bytes you control and trust! Passing unknown or
    // untrusted bytes will lead to arbitrary code execution vulnerabilities in
    // your system!
    let component_bytes  = include_bytes!("path/to/add_wasm32-wasip1.cwasm");

    let component = match unsafe { Component::deserialize_raw(&engine, component_bytes.as_slice().into()) } {
        Ok(comp) => comp,
        Err(error) => {
            println!("failed to deserialize pre-compiled module: {:?}", defmt::Debug2Format(&error));
            return Ok(());
        }
    };
    println!("Deserialized Component");
    // Instantiate the module and invoke its `add` function!
    let mut store = Store::new(&engine, ());
    let linker = component::Linker::new(&engine);

    bindgen!({
        world: "example",
        path: "../add/wit/world.wit",
        async: false,
    });
    println!("Generated Bindings");

    let bindings = Example::instantiate(&mut store, &component, &linker)?;
    let hello_world_string= bindings.call_hello_world(&mut store)?;
    println!("Wow i got {} from a Wasmtime Component", hello_world_string.as_str());
    let sum = bindings.call_add_two(&mut store, 3, 8)?;
    println!("the sum of 3 and 8 is {}", sum);

    Ok(())
}




/// Same as https://github.com/bytecodealliance/wasmtime/blob/main/examples/min-platform/embedding/wasmtime-platform.c
/// I have no idea whether this is safe.
/// https://github.com/bytecodealliance/wasmtime/blob/aec935f2e746d71934c8a131be15bbbb4392138c/crates/wasmtime/src/runtime/vm/traphandlers.rs#L888
//
static mut TLS_PTR: u32 = 0;
#[unsafe(no_mangle)]
extern "C" fn wasmtime_tls_get() -> *mut u8 {
    info!("Tries to get the tls ptr val");
    unsafe { TLS_PTR as *mut u8 }
}

#[unsafe(no_mangle)]
extern "C" fn wasmtime_tls_set(val: *const u8) {
   unsafe { TLS_PTR = val as u32 };
}
