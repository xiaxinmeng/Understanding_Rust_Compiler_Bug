
[01:09:47] ---- [run-pass] run-pass-fulldeps/issue-15149.rs stdout ----
[01:09:47] 
[01:09:47] error: test run failed!
[01:09:47] status: exit code: 101
[01:09:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-15149/a"
[01:09:47] stdout:
[01:09:47] ------------------------------------------
[01:09:47] 
[01:09:47] ------------------------------------------
[01:09:47] stderr:
[01:09:47] ------------------------------------------
[01:09:47] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', libcore/result.rs:1009:5
[01:09:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:09:47] 
[01:09:47] ------------------------------------------
[01:09:47] 
[01:09:47] thread '[run-pass] run-pass-fulldeps/issue-15149.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:09:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:09:47] 
[01:09:47] 
[01:09:47] failures:
[01:09:47]     [run-pass] run-pass-fulldeps/issue-15149.rs
