
% rustc --version
rustc 1.49.0-nightly (ffa2e7ae8 2020-10-24)
% RUSTFLAGS="-C target-feature=+crt-static" cargo test
error: cannot produce proc-macro for `ctor v0.1.16 (/home/otavio/src/rust-ctor/ctor)` as the target `x86_64-unknown-linux-gnu` does not support these crate types
