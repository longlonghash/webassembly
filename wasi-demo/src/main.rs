use std::env;
use std::fs;
use std::io::{Read, Write};

fn process(input_fname: &str, output_fname: &str) -> Result<(), String> {
    let mut input_file =
        fs::File::open(input_fname).map_err(|err| format!("error opening input {}: {}", input_fname, err))?;
    let mut contents = Vec::new();
    input_file
        .read_to_end(&mut contents)
        .map_err(|err| format!("read error: {}", err))?;

    let mut output_file = fs::File::create(output_fname)
        .map_err(|err| format!("error opening output {}: {}", output_fname, err))?;
    output_file
        .write_all(&contents)
        .map_err(|err| format!("write error: {}", err))
}

/*

## https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md

cargo build --target wasm32-wasi
file target/wasm32-wasi/debug/wasi-demo.wasm
echo "hello world" > test.txt

## sandboxing: giving it the capability to access a file by name
wasmtime --dir=. target/wasm32-wasi/debug/wasi-demo.wasm test.txt foo.txt
wasmtime --dir=. --dir=/tmp target/wasm32-wasi/debug/wasi-demo.wasm test.txt /tmp/bar.txt

## This maps the name /tmp within the WebAssembly program to /var/tmp in the host filesystem.
## So the WebAssembly program itself never sees the /var/tmp path, but that's where the output file goes.
wasmtime --dir=. --mapdir=/tmp::/var/tmp demo.wasm test.txt /tmp/somewhere.txt
cat /var/tmp/somewhere.txt

wasmtime demo.wat
wat2wasm demo.wat
wasmtime demo.wasm

*/


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("usage: {} <input_file> <output_file>", program);
        return;
    }

    if let Err(err) = process(&args[1], &args[2]) {
        eprintln!("{}", err)
    }
}

