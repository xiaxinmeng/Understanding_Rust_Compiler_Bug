
[00:50:03] failures:
[00:50:03] 
[00:50:03] ---- [run-pass] run-pass/multi-panic.rs stdout ----
[00:50:03] 	
[00:50:03] error: test run failed!
[00:50:03] status: exit code: 101
[00:50:03] command: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/multi-panic.stage2-x86_64-unknown-linux-gnu 
[00:50:03] stdout:
[00:50:03] ------------------------------------------
[00:50:03] 
[00:50:03] ------------------------------------------
[00:50:03] stderr:
[00:50:03] ------------------------------------------
[00:50:03] thread 'main' panicked at 'assertion failed: `(left == right)` (left: `Some("note: Run with `RUST_BACKTRACE=full` for a backtrace.")`, right: `Some("note: Run with `RUST_BACKTRACE=1` for a backtrace.")`)', /checkout/src/test/run-pass/multi-panic.rs:19
[00:50:03] note: Run with `RUST_BACKTRACE=full` for a backtrace.
[00:50:03] 
[00:50:03] ------------------------------------------
[00:50:03] 
[00:50:03] thread '[run-pass] run-pass/multi-panic.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2472
[00:50:03] note: Run with `RUST_BACKTRACE=full` for a backtrace.
[00:50:03] 
[00:50:03] 
[00:50:03] failures:
[00:50:03]     [run-pass] run-pass/multi-panic.rs
[00:50:03] 
[00:50:03] test result: FAILED. 2682 passed; 1 failed; 5 ignored; 0 measured
