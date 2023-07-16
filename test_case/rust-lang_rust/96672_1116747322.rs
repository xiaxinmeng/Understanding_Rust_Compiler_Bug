bash
cargo bisect-rustc --preserve --start=2022-01-13 --target=wasm32-unknown-unknown -- build --release --target=wasm32-unknown-unknown 
