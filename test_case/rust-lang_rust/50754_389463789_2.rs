sh
rustc --target=wasm32-unknown-unknown --crate-type=cdylib -Copt-level=1 src/lib.rs
node 1.js
