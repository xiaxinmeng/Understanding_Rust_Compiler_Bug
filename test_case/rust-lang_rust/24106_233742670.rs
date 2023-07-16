
$ rustc bar.rs -L .
foo.rs:5:19: 5:30 error: unimplemented constant expression: enum variants
foo.rs:5
                           ^~~~~~~~~~~
error: internal compiler error: ../src/librustc_trans\_match.rs:244: compare_list_exprs: type mismatch
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:619
note: Run with `RUST_BACKTRACE=1` for a backtrace.
