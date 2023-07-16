
---- [run-pass] run-pass/optimization-fuel-1.rs stdout ----
diff of stderr:

-	optimization-fuel-exhausted: Reorder fields of "S2"
-	


error: failed to delete `/home/wesley/code/rust/rust3/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-1/optimization-fuel-1.stderr`: No such file or directory (os error 2)
thread '[run-pass] run-pass/optimization-fuel-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2022:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [run-pass] run-pass/optimization-fuel-0.rs stdout ----
diff of stderr:

-	optimization-fuel-exhausted: Reorder fields of "S1"
-	


error: failed to delete `/home/wesley/code/rust/rust3/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-0/optimization-fuel-0.stderr`: No such file or directory (os error 2)
thread '[run-pass] run-pass/optimization-fuel-0.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2022:9


failures:
    [run-pass] run-pass/optimization-fuel-0.rs
    [run-pass] run-pass/optimization-fuel-1.rs

