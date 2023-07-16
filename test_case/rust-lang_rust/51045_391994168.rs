
[00:51:37] ---- [run-pass] run-pass/issue-50811.rs stdout ----
[00:51:37] 	
[00:51:37] error: test run failed!
[00:51:37] status: exit code: 101
[00:51:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-50811.stage2-x86_64-unknown-linux-gnu"
[00:51:37] stdout:
[00:51:37] ------------------------------------------
[00:51:37] 
[00:51:37] ------------------------------------------
[00:51:37] stderr:
[00:51:37] ------------------------------------------
[00:51:37] thread 'main' panicked at 'assertion failed: `(left == right)`
[00:51:37]   left: `false`,
[00:51:37]  right: `true`: -0.0 == 0.0', /checkout/src/test/run-pass/issue-50811.rs:59:5
[00:51:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:37] 
[00:51:37] ------------------------------------------
[00:51:37] 
[00:51:37] thread '[run-pass] run-pass/issue-50811.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:51:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:37] 
[00:51:37] 
[00:51:37] failures:
[00:51:37]     [run-pass] run-pass/issue-50811.rs
