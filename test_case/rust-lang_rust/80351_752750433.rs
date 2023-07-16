
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/errors/mod.rs:536
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: ../src/librustc_trans/common.rs:1102: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@<source>:2:22: 2:34] as std::ops::FnOnce<(<() as Query<'_>>::Result,)>>), Binder(<[closure@<source>:2:22: 2:34] as std::ops::FnOnce<((),)>>), Sorts(ExpectedFound { expected: (), found: <() as Query<'_>>::Result }))` selecting `Binder(<[closure@<source>:2:22: 2:34] as std::ops::FnOnce<((),)>>)` during trans
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
