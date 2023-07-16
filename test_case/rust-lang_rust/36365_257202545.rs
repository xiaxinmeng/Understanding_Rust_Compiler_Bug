
---- [run-pass] run-pass/iter-step-overflow-ndebug.rs stdout ----

error: test run failed!
status: exit code: 101
command: /build/build/x86_64-unknown-linux-gnu/test/run-pass/iter-step-overflow-ndebug.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)` (left: `255`, right: `0`)', /build/src/test/run-pass/iter-step-overflow-ndebug.rs:15
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

thread '[run-pass] run-pass/iter-step-overflow-ndebug.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2377
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass/iter-step-overflow-ndebug.rs
