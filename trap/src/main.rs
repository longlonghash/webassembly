
use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let store = Store::default();

    let module = Module::new(
        store.engine(),
        r#"
        (module
            (import "" "" (func $add (param i32 i32) (result i32)))
            (func (export "foo") (param i32 i32) (result i32)
                local.get 0
                local.get 1
                call $add))
    "#,
    )?;

    let add = Func::wrap(&store, |a: i32, b: i32| {
        match a.checked_add(b) {
            Some(i) => Ok(i),
            None => Err(Trap::new("overflow")),
        }
    });

    let instance = Instance::new(&store, &module, &[add.into()])?;

    let foo = instance.get_typed_func::<(i32, i32), i32>("foo")?;

    assert_eq!(foo.call((1, 2))?, 3);

    let ret = foo.call((i32::MAX, 1));
    assert!(ret.is_err());
    println!("ret: {:?}", ret);
    // println!("ret: {:#?}", ret);

    Ok(())
}
