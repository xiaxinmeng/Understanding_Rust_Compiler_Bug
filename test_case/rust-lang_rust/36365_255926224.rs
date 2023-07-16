
failures:
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:298

---- [run-fail] run-fail/iter-step-overflow-debug.rs stdout ----

error: failure produced the wrong error: exit code: 0
status: exit code: 0
command: /build/build/x86_64-unknown-linux-gnu/test/run-fail/iter-step-overflow-debug.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'attempt to add with overflow', src/libcore/ops.rs:315
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread 'main' panicked at 'attempt to add with overflow', src/libcore/ops.rs:315

------------------------------------------

thread '[run-fail] run-fail/iter-step-overflow-debug.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2377
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [run-fail] run-fail/iter-step-overflow-ndebug.rs stdout ----

error: no error pattern specified in "/build/src/test/run-fail/iter-step-overflow-ndebug.rs"
thread '[run-fail] run-fail/iter-step-overflow-ndebug.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1588


failures:
    [run-fail] run-fail/iter-step-overflow-debug.rs
    [run-fail] run-fail/iter-step-overflow-ndebug.rs
