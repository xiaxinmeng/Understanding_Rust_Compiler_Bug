ini
[00:52:03] failures:
[00:52:03] 
[00:52:03] ---- [run-fail] run-fail/assert-eq-macro-panic.rs stdout ----
[00:52:03] 	
[00:52:03] error: error pattern 'assertion failed: `(left == right)` (left: `14`, right: `15`)' not found!
[00:52:03] status: exit code: 101
[00:52:03] command: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/assert-eq-macro-panic.stage2-x86_64-unknown-linux-gnu 
[00:52:03] stdout:
[00:52:03] ------------------------------------------
[00:52:03] 
[00:52:03] ------------------------------------------
[00:52:03] stderr:
[00:52:03] ------------------------------------------
[00:52:03] thread 'main' panicked at 'assertion failed: `(left == right)`
[00:52:03] left:  `14`
[00:52:03] right: `15`', /checkout/src/test/run-fail/assert-eq-macro-panic.rs:14
[00:52:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:03] 
[00:52:03] ------------------------------------------
[00:52:03] 
[00:52:03] thread '[run-fail] run-fail/assert-eq-macro-panic.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2480
[00:52:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:03] 
[00:52:03] 
[00:52:03] failures:
[00:52:03]     [run-fail] run-fail/assert-eq-macro-panic.rs
[00:52:03] 
[00:52:03] test result: FAILED. 125 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
