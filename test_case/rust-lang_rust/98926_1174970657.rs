plain
C:\Users\WDAGUtilityAccount>rustc --version --verbose
rustc 1.64.0-nightly (27eb6d701 2022-07-04)
binary: rustc
commit-hash: 27eb6d7018e397cf98d51c205e3576951d766323
commit-date: 2022-07-04
host: x86_64-pc-windows-gnu
release: 1.64.0-nightly
LLVM version: 14.0.6

C:\Users\WDAGUtilityAccount>type test.rs
fn main() {
    panic!("");
}

C:\Users\WDAGUtilityAccount>rustc test.rs

C:\Users\WDAGUtilityAccount>test
thread 'main' panicked at '', test.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
