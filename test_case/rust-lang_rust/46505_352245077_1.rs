console
$ ls /cores

$ cargo new --bin rust-foo && cd rust-foo
     Created binary (application) `rust-foo` project

$ cargo build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26 secs

$ ulimit -c unlimited
$ touch src/main.rs && time cargo +stable build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
cargo +stable build  0.27s user 0.16s system 93% cpu 0.467 total

$ ls /cores

$ touch src/main.rs && time cargo +nightly build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.51 secs
cargo +nightly build  0.24s user 1.03s system 75% cpu 1.679 total

$ ls /cores
core.68358

$ ulimit -c 0
$ touch src/main.rs && time cargo +stable build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
cargo +stable build  0.27s user 0.17s system 94% cpu 0.466 total

$ touch src/main.rs && time cargo +nightly build
   Compiling rust-foo v0.1.0 (file:///Users/gib/tmp/rust-foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
cargo +nightly build  0.24s user 0.15s system 93% cpu 0.429 total

$ cd .. && rm -rf rust-foo
