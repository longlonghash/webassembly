extern crate wasmtime;
use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let store = Store::new(&engine);
    let module = Module::from_file(&engine, "examples/log.wat")?;
    /****
    let module = Module::new(
        &engine,
        r#"
        (module
            (import "" "log" (func $log (param i32)))
            (import "" "double" (func $double (param i32) (result i32)))
            (func (export "run")
                i32.const 0
                call $log i32.const 1
                call $log i32.const 2
                call $double call $log
            )
        )
        "#
    )?;
    ****/

    // First we can create our `log` function, which will simply print out the
    // parameter it receives.
    let log = Func::wrap(&store, |param: i32| {
        println!("log: {}", param);
    });

    // Next we can create our double function which doubles the input it receives.
    let double = Func::wrap(&store, |param: i32| param * 2);

    // When instantiating the module we now need to provide the imports to the
    // instantiation process. This is the second slice argument, where each
    // entry in the slice must line up with the imports in the module.
    let instance = Instance::new(&store, &module, &[log.into(), double.into()])?;

    let run = instance.get_typed_func::<(), ()>("run")?;
    Ok(run.call(())?)
}
