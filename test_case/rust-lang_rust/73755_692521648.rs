
RUSTFLAGS="-C target-feature=+multivalue,+reference-types,+bulk-memory -C panic=unwind" cargo build --target wasm32-unknown-unknown -Z build-std
