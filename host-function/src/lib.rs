
#![allow(unused)]
fn host_func() {
    struct MyStruct;
    #[link(wasm_import_module = "the-wasm-import-module")]
    extern "C" {
        // imports the name `foo` from `the-wasm-import-module`
        fn foo();

        // functions can have integer/float arguments/return values
        fn translate(a: i32) -> f32;

        // Note that the ABI of Rust and wasm is somewhat in flux, so while this
        // works, it's recommended to rely on raw integer/float values where
        // possible.
        fn translate_fancy(my_struct: MyStruct) -> u32;

        // you can also explicitly specify the name to import, this imports `bar`
        // instead of `baz` from `the-wasm-import-module`.
        #[link_name = "bar"]
        fn baz();
    }
}
