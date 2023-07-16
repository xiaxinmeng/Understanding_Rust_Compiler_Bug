
λ rustup run nightly rustc --version
rustc 1.43.0-nightly (6d0e58bff 2020-02-23)
λ rustup run nightly cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/rust-segfault`
[1]    24331 segmentation fault  rustup run nightly cargo run --release
