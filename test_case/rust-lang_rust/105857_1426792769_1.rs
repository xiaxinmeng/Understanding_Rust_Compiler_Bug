
rustc --crate-type=lib --emit=llvm-ir -C opt-level=3 -C no-prepopulate-passes --target=aarch64-unknown-linux-gnu fibonacci_vec.rs
