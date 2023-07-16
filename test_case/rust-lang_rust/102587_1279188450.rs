bash
$ cat hello-world.rs
#![feature(unix_sigpipe)]
#[unix_sigpipe = "sig_dfl"]
fn main() {
    loop {
        println!("hello world")
    }
}

$ RUSTC_BOOTSTRAP=1 ./build/aarch64-apple-darwin/stage0/bin/rustc hello-world.rs

$ ./hello-world | head -n 1
hello world

$ ./build/aarch64-apple-darwin/stage0/bin/rustc -V
rustc 1.65.0-beta.1 (2a65764f2 2022-09-19)
