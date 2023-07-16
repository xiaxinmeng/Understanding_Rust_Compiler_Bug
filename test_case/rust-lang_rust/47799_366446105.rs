
[01:17:06] ---- cargo_bench_failing_test stdout ----
[01:17:06] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
[01:17:06] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t13/foo/target/debug/foo`
[01:17:06] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench -- --test-threads=1`
[01:17:06] thread 'cargo_bench_failing_test' panicked at '
[01:17:06] Expected: execs
[01:17:06]     but: expected to find:
[01:17:06] test bench_hello ... 
[01:17:06] 
[01:17:06] did not find in output:
[01:17:06] 
[01:17:06] running 1 test
[01:17:06] test bench_hello ... FAILED
[01:17:06] 
[01:17:06] failures:
[01:17:06] 
[01:17:06] ---- bench_hello stdout ----
[01:17:06] <tab>thread 'main' panicked at 'assertion failed: `(left == right)`
[01:17:06]   left: `"hello"`,
[01:17:06]  right: `"nope"`', src/main.rs:15:17
[01:17:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:17:06] 
[01:17:06] 
[01:17:06] failures:
[01:17:06]     bench_hello
[01:17:06] 
[01:17:06] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:17:06] 
[01:17:06] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31:13
[01:17:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:17:06] 
[01:17:06] 
[01:17:06] failures:
[01:17:06]     cargo_bench_failing_test
