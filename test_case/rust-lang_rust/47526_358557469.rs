
rustc --target=wasm32-unknown-unknown --emit llvm-ir -C lto -C opt-level=3 -Z thinlto=off src/main.rs
