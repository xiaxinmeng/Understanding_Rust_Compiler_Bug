
[01:50:10] ---- [run-pass] run-pass-fulldeps/issue-15149.rs stdout ----
[01:50:10] 	
[01:50:10] error: test run failed!
[01:50:10] status: exit code: 101
[01:50:10] command: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-pass-fulldeps/issue-15149.stage2-i686-apple-darwin"
[01:50:10] stdout:
[01:50:10] ------------------------------------------
[01:50:10] 
[01:50:10] ------------------------------------------
[01:50:10] stderr:
[01:50:10] ------------------------------------------
[01:50:10] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', libcore/result.rs:945:5
[01:50:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:50:10] 
[01:50:10] ------------------------------------------
[01:50:10] 
[01:50:10] thread '[run-pass] run-pass-fulldeps/issue-15149.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2893:9
[01:50:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:50:10] 
[01:50:10] 
[01:50:10] failures:
[01:50:10]     [run-pass] run-pass-fulldeps/issue-15149.rs
