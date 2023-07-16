console
$ cargo new --lib a
$ cd a
$ rustc -Z unstable-options --target bpfel-unknown-none --print target-spec-json | grep -v is-builtin > bpfel-unknown-none.json
$ sed -i 's/"arch": "bpf",/"arch": "bpf","atomic-cas": false,/' bpfel-unknown-none.json
$ echo '#![no_std]' > src/lib.rs
$ cargo build -Z build-std=core --target bpfel-unknown-none.json
error: cannot find macro `atomic_int` in this scope
    --> /Users/taiki/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2857:1
     |
2857 | atomic_int! {
     | ^^^^^^^^^^

...
