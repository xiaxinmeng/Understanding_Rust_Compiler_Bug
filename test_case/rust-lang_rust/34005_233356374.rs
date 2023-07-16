
failures:

---- [run-pass] run-pass/compiler-calls.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: the input file would be overwritten by the generated executable
thread 'main' panicked at 'assertion failed: `(left == right)` (left: `10`, right: `30`)', /build/src/test/run-pass-fulldeps/compiler-calls.rs:86
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

thread '[run-pass] run-pass/compiler-calls.rs' panicked at 'explicit panic', /build/src/tools/compiletest/src/runtest.rs:2243
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass/compiler-calls.rs

test result: FAILED. 58 passed; 1 failed; 0 ignored; 0 measured
