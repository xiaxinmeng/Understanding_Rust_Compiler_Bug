console
$ cargo new --bin rust-foo && cd rust-foo
     Created binary (application) `rust-foo` project
Cargo.toml src

$ cargo build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.92 secs

$ touch src/main.rs && time cargo +stable build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39 secs
cargo +stable build  0.37s user 0.22s system 92% cpu 0.639 total

$ touch src/main.rs && time cargo +nightly build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.74 secs
cargo +nightly build  0.34s user 1.06s system 47% cpu 2.995 total
