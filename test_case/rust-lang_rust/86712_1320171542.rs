console
$ rustc +nightly --version -v
rustc 1.67.0-nightly (77e57db38 2022-10-30)
binary: rustc
commit-hash: 77e57db384aca99444c3b5f6a9c86bc58a804d89
commit-date: 2022-10-30
host: x86_64-unknown-linux-gnu
release: 1.67.0-nightly
LLVM version: 15.0.4
$ git clone https://github.com/rocallahan/rust-musl-segfault.git
$ cd rust-musl-segfault/
$ RUSTFLAGS="-Ctarget-feature=+crt-static -Clinker=rust-lld" cargo +nightly build --target x86_64-unknown-linux-musl
$ ./target/x86_64-unknown-linux-musl/debug/test_crate
$ echo $?
0
