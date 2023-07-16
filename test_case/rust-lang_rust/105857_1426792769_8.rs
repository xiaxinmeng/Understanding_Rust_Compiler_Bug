
rustc --crate-type=lib --emit=llvm-ir -C opt-level=3 -C llvm-args=-unroll-runtime --target=aarch64-unknown-linux-gnu fibonacci_vec.rs -o fibonacci_vec.unroll-runtime.ll
