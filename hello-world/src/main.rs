//! Small example of how to instantiate a wasm module that imports one function,
//! showing how you can fill in host functionality for a wasm module.

// You can execute this example with `cargo run --example hello`

use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    // Configure the initial compilation environment, creating the global
    // `Store` structure. Note that you can also tweak configuration settings
    // with a `Config` and an `Engine` if desired.
    println!("Initializing...");
    let store = Store::default();

    // Compile the wasm binary into an in-memory instance of a `Module`.
    println!("Compiling module...");
    let module = Module::from_file(store.engine(), "examples/hello.wat")?;

    // Here we handle the imports of the module, which in this case is our
    // `HelloCallback` type and its associated implementation of `Callback.
    println!("Creating callback...");
    let hello_func = Func::wrap(&store, || {
        println!("Calling back...");
        println!("> Hello World!");
    });

    // Once we've got that all set up we can then move to the instantiation
    // phase, pairing together a compiled module as well as a set of imports.
    // Note that this is where the wasm `start` function, if any, would run.
    println!("Instantiating module...");
    let imports = [hello_func.into()];
    let instance = Instance::new(&store, &module, &imports)?;

    // Next we poke around a bit to extract the `run` function from the module.
    println!("Extracting export...");
    let run = instance.get_typed_func::<(), ()>("run")?;

    // And last but not least we can call it!
    println!("Calling export...");
    run.call(())?;

    println!("Done.");
    Ok(())
}


fn main1() -> Result<()> {
    // All wasm objects operate within the context of a "store"
    let store = Store::default();

    // Modules can be compiled through either the text or binary format
    let wat = r#"
        (module
            (import "" "" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
        "#;
    let module = Module::new(store.engine(), wat)?;

    // Host functions can be defined which take/return wasm values and
    // execute arbitrary code on the host.
    let host_hello = Func::wrap(&store, |param: i32| {
        println!("Got {} from WebAssembly", param);
    });

    // Instantiation of a module requires specifying its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    let instance = Instance::new(&store, &module, &[host_hello.into()])?;
    let hello = instance.get_typed_func::<(), ()>("hello")?;

    // And finally we can call the wasm as if it were a Rust function!
    hello.call(())?;

    Ok(())
}

