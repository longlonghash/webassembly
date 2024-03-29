extern crate wasmtime;
use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    // A `Store` is a sort of "global object" in a sense, but for now it suffices
    // to say that it's generally passed to most constructors.
    let store = Store::new(&engine);

    if false {
        // We start off by creating a `Module` which represents a compiled form
        // of our input wasm module. In this case it'll be JIT-compiled after
        // we parse the text format.
        let module = Module::from_file(&engine, "examples/answer.wat")?;
    }
    let module = Module::new(&engine, r#"(module (func (export "answer") (result i32) i32.const 42))"#)?;

    // After we have a compiled `Module` we can then instantiate it, creating
    // an `Instance` which we can actually poke at functions on.
    let instance = Instance::new(&store, &module, &[])?;

    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.
    let answer = instance.get_func("answer")
        .expect("`answer` was not an exported function");

    // There's a few ways we can call the `answer` `Func` value. The easiest
    // is to statically assert its signature with `typed` (in this case
    // asserting it takes no arguments and returns one i32) and then call it.
    let answer = answer.typed::<(), i32>()?;

    // And finally we can call our function! Note that the error propagation
    // with `?` is done to handle the case where the wasm function traps.
    let result = answer.call(())?;
    println!("Answer: {:?}", result);
    Ok(())
}
