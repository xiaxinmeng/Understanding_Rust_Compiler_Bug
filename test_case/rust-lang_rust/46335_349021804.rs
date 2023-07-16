
[01:01:07] ---- [run-pass] run-pass-fulldeps/issue-35829.rs stdout ----
[01:01:07] 	
[01:01:07] error: test run failed!
[01:01:07] status: exit code: 101
[01:01:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-35829.stage2-x86_64-unknown-linux-gnu"
[01:01:07] stdout:
[01:01:07] ------------------------------------------
[01:01:07] 
[01:01:07] ------------------------------------------
[01:01:07] stderr:
[01:01:07] ------------------------------------------
[01:01:07] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:331:21
[01:01:07] thread 'main' panicked at 'position 0 does not resolve to a source location', /checkout/src/libsyntax/codemap.rs:632:8
[01:01:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:07] 
[01:01:07] ------------------------------------------
[01:01:07] 
[01:01:07] thread '[run-pass] run-pass-fulldeps/issue-35829.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2570:8
[01:01:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:07] 
[01:01:07] ---- [run-pass] run-pass-fulldeps/qquote.rs stdout ----
[01:01:07] 	
[01:01:07] error: test run failed!
[01:01:07] status: exit code: 101
[01:01:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/qquote.stage2-x86_64-unknown-linux-gnu"
[01:01:07] stdout:
[01:01:07] ------------------------------------------
[01:01:07] 
[01:01:07] ------------------------------------------
[01:01:07] stderr:
[01:01:07] ------------------------------------------
[01:01:07] thread 'main' panicked at 'position 0 does not resolve to a source location', /checkout/src/libsyntax/codemap.rs:632:8
[01:01:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:07] 
[01:01:07] ------------------------------------------
[01:01:07] 
[01:01:07] thread '[run-pass] run-pass-fulldeps/qquote.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2570:8
[01:01:07] 
[01:01:07] 
[01:01:07] failures:
[01:01:07]     [run-pass] run-pass-fulldeps/issue-35829.rs
[01:01:07]     [run-pass] run-pass-fulldeps/qquote.rs
[01:01:07] 
[01:01:07] test result: FAILED. 84 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
