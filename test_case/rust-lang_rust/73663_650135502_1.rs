
% rustc test.rs
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_resolve/late.rs:2124:52
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-nightly (50fc24d8a 2020-06-25) running on x86_64-unknown-linux-gnu
