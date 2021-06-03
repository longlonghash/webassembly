//! Example of instantiating of the WebAssembly module and invoking its exported
//! function.

// You can execute this example with `cargo run --example gcd`

use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let store = Store::default();

    let module = Module::new(
        store.engine(),
        r#"
        (module
            (import "" "" (func $debug (param i32 i32 f32 i64 i64 f64)))
            (func (export "foo")
                i32.const -1
                i32.const 1
                f32.const 2
                i64.const -3
                i64.const 3
                f64.const 4
                call $debug))
    "#,
    )?;

    let debug = Func::wrap(&store, |a: i32, b: u32, c: f32, d: i64, e: u64, f: f64| {

        println!("a={}", a);
        println!("b={}", b);
        println!("c={}", c);
        println!("d={}", d);
        println!("e={}", e);
        println!("f={}", f);
    });

    let instance = Instance::new(&store, &module, &[debug.into()])?;

    let foo = instance.get_typed_func::<(), ()>("foo")?;

    foo.call(())?;

    Ok(())
}
