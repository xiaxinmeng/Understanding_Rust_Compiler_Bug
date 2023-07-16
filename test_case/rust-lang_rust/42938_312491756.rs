
[01:12:30] ---- cargo_test_failing_test_in_lib stdout ----
[01:12:30] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo test`
[01:12:30] thread 'cargo_test_failing_test_in_lib' panicked at '
[01:12:30] Expected: execs
[01:12:30]     but: expected to find:
[01:12:30] test test_hello ... FAILED
[01:12:30] 
[01:12:30] failures:
[01:12:30] 
[01:12:30] ---- test_hello stdout ----
[01:12:30] <tab>thread 'test_hello' panicked at 'assertion failed: false', src[/]lib.rs:4
[01:12:30] 
[01:12:30] 
[01:12:30] did not find in output:
[01:12:30] 
[01:12:30] running 1 test
[01:12:30] test test_hello ... FAILED
[01:12:30] 
[01:12:30] failures:
[01:12:30] 
[01:12:30] ---- test_hello stdout ----
[01:12:30] <tab>thread 'test_hello' panicked at 'assertion failed: false', src/lib.rs:4:16
[01:12:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:30] 
[01:12:30] 
[01:12:30] failures:
[01:12:30]     test_hello
[01:12:30] 
[01:12:30] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:12:30] 
[01:12:30] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31:12
[01:12:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:30] 
[01:12:30] ---- cargo_test_failing_test_in_test stdout ----
[01:12:30] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
[01:12:30] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t11/foo/target/debug/foo`
[01:12:30] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo test`
[01:12:30] thread 'cargo_test_failing_test_in_test' panicked at '
[01:12:30] Expected: execs
[01:12:30]     but: expected to find:
[01:12:30] running 1 test
[01:12:30] test test_hello ... FAILED
[01:12:30] 
[01:12:30] failures:
[01:12:30] 
[01:12:30] ---- test_hello stdout ----
[01:12:30] <tab>thread 'test_hello' panicked at 'assertion failed: false', tests[/]footest.rs:4
[01:12:30] 
[01:12:30] 
[01:12:30] did not find in output:
[01:12:30] 
[01:12:30] running 0 tests
[01:12:30] 
[01:12:30] test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:12:30] 
[01:12:30] 
[01:12:30] running 1 test
[01:12:30] test test_hello ... FAILED
[01:12:30] 
[01:12:30] failures:
[01:12:30] 
[01:12:30] ---- test_hello stdout ----
[01:12:30] <tab>thread 'test_hello' panicked at 'assertion failed: false', tests/footest.rs:4:16
[01:12:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:30] 
[01:12:30] 
[01:12:30] failures:
[01:12:30]     test_hello
[01:12:30] 
[01:12:30] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:12:30] 
[01:12:30] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31:12
[01:12:30] 
[01:12:30] 
[01:12:30] failures:
[01:12:30]     cargo_test_failing_test_in_lib
[01:12:30]     cargo_test_failing_test_in_test
