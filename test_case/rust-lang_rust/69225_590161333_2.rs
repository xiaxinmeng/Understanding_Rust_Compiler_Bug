
$ rustup run nightly rustc --version
rustc 1.43.0-nightly (6d0e58bff 2020-02-23)
$ rustup run nightly cargo run --release
    Finished release [optimized] target(s) in 1.13s
     Running `target/release/rust-segfault`
Segmentation fault (core dumped)
