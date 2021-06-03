# webassembly

The Rust Programming Language supports WebAssembly as a compilation target. If you're not familiar with Rust it's recommended to start with its introductory documentation. Compiling to WebAssembly will involve specifying the desired target via the --target flag, and to do this there are a number of "target triples" for WebAssembly compilation in Rust:

## wasm32-wasi
when using wasmtime this is likely what you'll be using. The WASI target is integrated into the standard library and is intended on producing standalone binaries.
## wasm32-unknown-unknown 
this target, like the WASI one, is focused on producing single *.wasm binaries. The standard library, however, is largely stubbed out since the "unknown" part of the target means libstd can't assume anything. This means that while binaries will likely work in wasmtime, common conveniences like println! or panic! won't work.
## wasm32-unknown-emscripten
this target is intended to work in a web browser and produces a *.wasm file coupled with a *.js file, and it is not compatible with wasmtime.

