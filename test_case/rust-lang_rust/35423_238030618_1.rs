
$ rustc bug.rs
error: internal compiler error: src/librustc_trans/_match.rs:943: only string and byte strings supported in compare_values
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', src/libsyntax/errors/mod.rs:584
note: Run with `RUST_BACKTRACE=1` for a backtrace.
