

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/*

https://docs.wasmtime.dev/wasm-rust.html
cargo wasi build --release
## Using cargo wasi build, there might be some errors!!!

cargo build --target=wasm32-wasi --release
cargo build --target=wasm32-unknown-unknown --release

cargo +nightly build --target=wasm32-wasi --release
cargo +nightly build --target=wasm32-unknown-unknown --release
cargo +stable build --target=wasm32-wasi --release
cargo +stable build --target=wasm32-unknown-unknown --release

wasmtime --invoke greet ./target/wasm32-wasi/release/cdylib_bindgen.wasm "Wasmtime CLI"
## there might be some errors!!!


*/
