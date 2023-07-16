

[01:11:06] thread 'cargo_bench_failing_test' panicked at '
[01:11:06] Expected: execs
[01:11:06]     but: expected to find:
[01:11:06] [..]src[/]foo.rs:14
[01:11:06] 
[01:11:06] did not find in output:
[01:11:06]    Compiling foo v0.5.0 (file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t11/foo)
[01:11:06]     Finished release [optimized] target(s) in 0.67 secs
[01:11:06]      Running target/release/deps/foo-b865dce48c3e4e89
[01:11:06] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:11:06]   left: `"hello"`,
[01:11:06]  right: `"nope"`', src/foo.rs:14:16
[01:11:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:06] error: bench failed
[01:11:06] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31:12
