
$ cat ../foo/src/lib.rs 
pub fn foo(v: u8) {
    debug_assert!(v < 128);
    println!("{}", v);
}
$ cat src/main.rs 
extern crate foo;

fn main() {
    foo::foo(8);
    foo::foo(128);
}
$ cat Cargo.toml 
[package]
name = "bar"
version = "0.1.0"
authors = []

[dependencies]
foo = {path ="../foo"}

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/bar`
8
thread 'main' panicked at 'assertion failed: v < 128', /tmp/tmp.WvYvoFMNXk/foo/src/lib.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

$ cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/bar`
8
128
