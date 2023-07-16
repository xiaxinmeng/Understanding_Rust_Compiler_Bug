
touch lib.rs
rustc lib.rs -C link-arg=-sSIDE_MODULE=2 --target wasm32-unknown-emscripten --crate-type cdylib
