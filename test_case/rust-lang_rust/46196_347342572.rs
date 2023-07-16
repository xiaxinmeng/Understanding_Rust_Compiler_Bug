
[01:19:09] command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "--test" "/checkout/src/doc/rust-by-example/src/std_misc/process.md" "--test-args" ""
[01:19:09] expected success, got: exit code: 101
[01:19:09] 
[01:19:09] stdout ----
[01:19:09] 
[01:19:09] running 1 test
[01:19:09] test /checkout/src/doc/rust-by-example/src/std_misc/process.md - Child_processes (line 6) ... FAILED
[01:19:09] 
[01:19:09] failures:
[01:19:09] 
[01:19:09] ---- /checkout/src/doc/rust-by-example/src/std_misc/process.md - Child_processes (line 6) stdout ----
[01:19:09] 	thread 'rustc' panicked at 'test executable failed:
[01:19:09] 
[01:19:09] thread 'main' panicked at 'failed to execute process: No such file or directory (os error 2)', /checkout/src/doc/rust-by-example/src/std_misc/process.md:8:12
[01:19:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:19:09] 
[01:19:09] ', /checkout/src/librustdoc/test.rs:322:16
[01:19:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:19:09] 
[01:19:09] 
[01:19:09] failures:
[01:19:09]     /checkout/src/doc/rust-by-example/src/std_misc/process.md - Child_processes (line 6)
[01:19:09] 
[01:19:09] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:19:09] 
[01:19:09] 
[01:19:09] stderr ----
