
[00:18:48]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:18:49] error: internal compiler error: /checkout/src/librustc/ty/subst.rs:411: Region parameter out of range when substituting in region 'a (root type=None) (index=2)
[00:18:49] 
[00:18:49] note: the compiler unexpectedly panicked. this is a bug.
[00:18:49] 
[00:18:49] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:18:49] 
[00:18:49] note: rustc 1.21.0-dev (d09f53bc8 2017-08-12) running on x86_64-unknown-linux-gnu
[00:18:49] 
[00:18:49] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:434:8
[00:18:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:18:49] 
[00:18:49] error: Could not compile `rustc_data_structures`.
