
$ rustc +stable -C prefer-dynamic x.rs && rustup run stable ./x
$ rustc +beta -C prefer-dynamic x.rs && rustup run beta ./x
thread 'main' panicked at 'assertion failed: unsafe { HIT }', x.rs:21:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
