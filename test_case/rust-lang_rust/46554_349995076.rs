
[01:07:12] running 2 tests
[01:07:12] test compile_test ... FAILED
[01:07:12] test dogfood ... FAILED
[01:07:12] 
[01:07:12] failures:
[01:07:12] 
[01:07:12] ---- compile_test stdout ----
[01:07:12] 	thread 'compile_test' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 30, message: "Read-only file system" } }', /checkout/src/libcore/result.rs:906:4
[01:07:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:12] 
[01:07:12] ---- dogfood stdout ----
[01:07:12] 	thread 'dogfood' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 2, message: "No such file or directory" } }', /checkout/src/libcore/result.rs:906:4
[01:07:12] 
[01:07:12] 
[01:07:12] failures:
[01:07:12]     compile_test
[01:07:12]     dogfood
[01:07:12] 
[01:07:12] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:07:12] 
[01:07:12] error: test failed, to rerun pass '--test compile-test'
