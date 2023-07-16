
[01:05:40] ---- [run-pass] run-pass/vector-sort-panic-safe.rs stdout ----
[01:05:40]
[01:05:40] error: test run failed!
[01:05:40] status: exit code: 101
[01:05:40] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/remote-test-client run /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/vector-sort-panic-safe.stage2-arm-unknown-linux-gnueabihftdout:
[01:05:40] ------------------------------------------
[01:05:40] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/vector-sort-panic-safe.stage2-arm-unknown-linux-gnueabihf", waiting for result
[01:05:40]
[01:05:40] ------------------------------------------
[01:05:40] stderr:
[01:05:40] ------------------------------------------
[01:05:40] thread 'main' panicked at 'attempt to calculate the remainder with a divisor of zero', /checkout/src/test/run-pass/vector-sort-panic-safe.rs:152
[01:05:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:40]
[01:05:40] ------------------------------------------
[01:05:40]
[01:05:40] thread '[run-pass] run-pass/vector-sort-panic-safe.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2479
[01:05:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:40]
[01:05:40]
[01:05:40] failures:
[01:05:40]     [run-pass] run-pass/vector-sort-panic-safe.rs
[01:05:40]
[01:05:40] test result: FAILED. 2697 passed; 1 failed; 9 ignored; 0 measured; 0 filtered out
[01:05:40]
[01:05:40] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:325
