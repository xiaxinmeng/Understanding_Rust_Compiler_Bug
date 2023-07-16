`
WARNING: Ignoring `RUSTC_WRAPPER` environment variable, Miri does not support wrapping.
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/matthias/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri target/miri/x86_64-unknown-linux-gnu/debug/miri`
thread 'rustc' panicked at 'assertion failed: base.layout.ty.is_simd()', /rustc/6dd68402c5d7da168f87d8551dd9aed1d8a21893/compiler/rustc_const_eval/src/interpret/operand.rs:500:9
