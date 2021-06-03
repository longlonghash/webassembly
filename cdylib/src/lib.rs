
#[no_mangle]

pub extern "C" fn print_hello() {
    println!("Hello, world!");
}

/*

cd path/to/webassembly/cdylib
cargo wasi build
cd ../  ## webassembly
wasmtime --invoke print_hello target/wasm32-wasi/debug/cdylib.wasm


$ cp target/wasm32-wasi/debug/hello_world.wasm .
$ python3
>>> import wasmtime
>>> import hello_world
>>> hello_world.print_hello()
Hello, world!
()
>>>

*/

