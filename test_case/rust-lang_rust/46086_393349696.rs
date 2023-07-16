
[-@- rust]$ cat validate-fail.rs
fn main() { main2::<()>(); }
fn main2<T>() {
    let c = |_x| {};
    c(&mut 1);
}
[-@- rust]$ rustup run nightly rustc validate-fail.rs -Z mir-emit-validate=1 -Z mir-opt-level=2
[-@- rust]$ rustup run nightly rustc --version --verbose
rustc 1.28.0-nightly (524ad9b9e 2018-05-29)
binary: rustc
commit-hash: 524ad9b9e03656f3fdeb03ed82fe78db3916e566
commit-date: 2018-05-29
host: x86_64-unknown-linux-gnu
release: 1.28.0-nightly
LLVM version: 6.0
