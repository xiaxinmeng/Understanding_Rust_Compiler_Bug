
[01:06:58] failures:
[01:06:58] 
[01:06:58] ---- cargo_test_failing_test_in_bin stdout ----
[01:06:58] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
[01:06:58] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t9/foo/target/debug/foo`
[01:06:58] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo test`
[01:06:58] thread 'cargo_test_failing_test_in_bin' panicked at '
[01:06:58] Expected: execs
[01:06:58]     but: expected to find:
[01:06:58] [..]left: `"hello"`,[..]
[01:06:58] 
[01:06:58] did not find in output:
[01:06:58] 
[01:06:58] running 1 test
[01:06:58] test test_hello ... FAILED
[01:06:58] 
[01:06:58] failures:
[01:06:58] 
[01:06:58] ---- test_hello stdout ----
[01:06:58] <tab>thread 'test_hello' panicked at 'assertion failed: `(left == right)`
[01:06:58]   left: `"hello"`
[01:06:58]  right: `"nope"`', src/foo.rs:12
[01:06:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:58] 
[01:06:58] 
[01:06:58] failures:
[01:06:58]     test_hello
[01:06:58] 
[01:06:58] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:58] 
[01:06:58] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31
[01:06:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:58] 
[01:06:58] 
[01:06:58] failures:
[01:06:58]     cargo_test_failing_test_in_bin
[01:06:58] 
[01:06:58] test result: FAILED. 75 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
