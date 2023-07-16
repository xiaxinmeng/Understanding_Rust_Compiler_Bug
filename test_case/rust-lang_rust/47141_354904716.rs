
[00:06:01]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:14] error: internal compiler error: error writing metadata for triple `x86_64-unknown-linux-gnu` and crate `libsyntax`, error: other os error, cause: None
[00:06:14]    --> libsyntax/lib.rs:154:1
[00:06:14]     |
[00:06:14] 154 | __build_diagnostic_array! { libsyntax, DIAGNOSTICS }
[00:06:14]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:06:14] 
[00:06:14] note: the compiler unexpectedly panicked. this is a bug.
[00:06:14] 
[00:06:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:06:14] 
[00:06:14] note: rustc 1.24.0-beta.1 (5b496b726 2018-01-02) running on x86_64-unknown-linux-gnu
[00:06:14] 
[00:06:14] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:451:9
[00:06:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:06:14] 
[00:06:14] error: Could not compile `syntax`.
