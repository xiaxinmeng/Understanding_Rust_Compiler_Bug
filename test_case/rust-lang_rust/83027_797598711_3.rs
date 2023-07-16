
$ cat Cargo.toml
[package]
name = "memchr-perf-regression"
version = "0.1.0"
authors = ["Andrew Gallant <jamslam@gmail.com>"]
edition = "2018"

[dependencies]
memchr = "2"

$ cat src/main.rs
use memchr::memchr;

fn main() {
    let haystack = "abcdefghijklmnopqrstuvwxyz".repeat(15);

    for _ in 0..100_000_000 {
        assert_eq!(None, memchr(b'@', haystack.as_bytes()));
    }
}
