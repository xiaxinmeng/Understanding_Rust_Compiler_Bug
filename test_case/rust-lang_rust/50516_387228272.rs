
$ RUSTFLAGS="-C llvm-args=-fast-isel" cargo run
   Compiling dir-exists-aarch64 v0.1.0 (file:///home/yrashk/dir-exists-aarch64)
    Finished dev [unoptimized + debuginfo] target(s) in 2.73 secs
     Running `target/debug/dir-exists-aarch64`
thread 'main' panicked at '1: Os { code: 17, kind: AlreadyExists, message: "File exists" }', libcore/result.rs:945:5
note: Run with `RUST_BACKTRACE=1` for a backtrace
