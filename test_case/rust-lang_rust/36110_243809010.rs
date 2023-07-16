
seas779:cargo-script alex$ RUSTFLAGS=-Zorbit=off mr ru nightly-2016-08-23 cargo script --force --use-shared-binary-cache no --debug -e '(255u8..).next()'
   Compiling expr v0.1.0 (file:///Users/alex/.multirust/toolchains/nightly-2016-08-23/cargo/.cargo/script-cache/expr-24db503e5185c33f)
    Finished debug [unoptimized + debuginfo] target(s) in 0.41 secs
thread 'main' panicked at 'attempt to add with overflow', ../src/libcore/iter/range.rs:103
note: Run with `RUST_BACKTRACE=1` for a backtrace.
seas779:cargo-script alex$ RUSTFLAGS=-Zorbit=on mr ru nightly-2016-08-23 cargo script --force --use-shared-binary-cache no --debug -e '(255u8..).next()'
   Compiling expr v0.1.0 (file:///Users/alex/.multirust/toolchains/nightly-2016-08-23/cargo/.cargo/script-cache/expr-24db503e5185c33f)
    Finished debug [unoptimized + debuginfo] target(s) in 0.39 secs
Some(255)
