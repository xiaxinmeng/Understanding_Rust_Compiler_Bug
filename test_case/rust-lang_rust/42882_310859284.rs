
[01:06:00] 
[01:06:00] failures:
[01:06:00] 
[01:06:00] ---- [run-pass] run-pass/vector-sort-panic-safe.rs stdout ----
[01:06:00] 	
[01:06:00] error: test run failed!
[01:06:00] status: exit code: 101
[01:06:00] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/remote-test-client run /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/vector-sort-panic-safe.stage2-arm-unknown-linux-gnueabihf
[01:06:00] stdout:
[01:06:00] ------------------------------------------
[01:06:00] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/vector-sort-panic-safe.stage2-arm-unknown-linux-gnueabihf", waiting for result
[01:06:00] 
[01:06:00] ------------------------------------------
[01:06:00] stderr:
[01:06:00] ------------------------------------------
[01:06:00] thread 'main' panicked at 'attempt to calculate the remainder with a divisor of zero', /checkout/src/test/run-pass/vector-sort-panic-safe.rs:151
[01:06:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:00] 
[01:06:00] ------------------------------------------
[01:06:00] 
[01:06:00] thread '[run-pass] run-pass/vector-sort-panic-safe.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2480
[01:06:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:00] 
[01:06:00] 
[01:06:00] failures:
[01:06:00]     [run-pass] run-pass/vector-sort-panic-safe.rs
[01:06:00] 
[01:06:00] test result: [31mFAILED(B[m. 2696 passed; 1 failed; 9 ignored; 0 measured; 0 filtered out
