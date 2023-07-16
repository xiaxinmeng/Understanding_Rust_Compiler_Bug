
rustc --crate-name code lib.rs --crate-type cdylib --crate-type rlib -C opt-level=z -C codegen-units=1 --target=wasm32-unknown-unknown
