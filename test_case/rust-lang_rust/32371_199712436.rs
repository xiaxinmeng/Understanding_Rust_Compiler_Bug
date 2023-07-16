
$ rustc -O /tmp/nodakai/ice.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report:
note:   https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: version: 1.9.0-dev (1437edb98 2016-03-22)
note: host: x86_64-unknown-linux-gnu
note: arguments: ["rustc", "-O", "/tmp/nodakai/ice.rs"]
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'impossible range in AST', src/librustc_front/lowering.rs:1292
note: Run with `RUST_BACKTRACE=1` for a backtrace.
