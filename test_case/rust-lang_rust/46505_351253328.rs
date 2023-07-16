console
$ cargo new --bin rust-foo && cd rust-foo
     Created binary (application) `rust-foo` project
$ cargo build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.92 secs
$ touch src/main.rs && time cargo +stable build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
cargo +stable build  0.26s user 0.17s system 92% cpu 0.466 total
$ touch src/main.rs && time cargo +nightly build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.57 secs
cargo +nightly build  0.31s user 0.22s system 68% cpu 0.774 total
