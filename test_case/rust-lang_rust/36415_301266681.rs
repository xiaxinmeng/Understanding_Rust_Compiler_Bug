
$ rustc -vV | test
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'failed printing to stdout: Broken pipe (os error 32)', src/libstd/io/stdio.rs:691
note: Run with `RUST_BACKTRACE=1` for a backtrace.
