
------------------------------------------
stderr:
------------------------------------------
error: Unrecognized option: 'invalid-arg-foo'
error: kaboom
  --> compile-error.rs:12:5
   |
12 |     compile_error!("kaboom");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
error: aborting due to previous error
error: kaboom
  --> compile-error.rs:12:5
   |
12 |     compile_error!("kaboom");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
thread 'main' panicked at 'encountered error with `-Z treat_err_as_bug', librustc_errors\lib.rs:478:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.30.0-dev running on x86_64-pc-windows-msvc
note: compiler flags: -Z treat-err-as-bug
error: Unrecognized option: 'invalid-arg-foo'
error: kaboom
  --> compile-error.rs:12:5
   |
12 |     compile_error!("kaboom");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
make: *** [Makefile:11: all] Error 1
------------------------------------------
thread '[run-make] run-make-fulldeps\exit-code' panicked at 'explicit panic', tools\compiletest\src\runtest.rs:3149:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    [run-make] run-make-fulldeps\exit-code
test result: FAILED. 187 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
