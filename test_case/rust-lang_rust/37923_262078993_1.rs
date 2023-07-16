
$ rustup run nightly cargo --version
cargo 0.16.0-nightly (d1bfc01 2016-11-18)
$ rustup run nightly rustc --version --verbose
rustc 1.15.0-nightly (0bd2ce62b 2016-11-19)
binary: rustc
commit-hash: 0bd2ce62b27e2b9a7dfe92fc23d9098854008089
commit-date: 2016-11-19
host: x86_64-unknown-linux-gnu
release: 1.15.0-nightly
LLVM version: 3.9
$ grep clippy ~/.cargo/.crates.toml
"clippy 0.0.100 (registry+https://github.com/rust-lang/crates.io-index)" = ["cargo-clippy"]
