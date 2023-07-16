
[01:04:38] ---- cargo_bench_failing_test stdout ----

[01:04:38] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`

[01:04:38] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t11/foo/target/debug/foo`

[01:04:38] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench`

[01:04:38] thread 'cargo_bench_failing_test' panicked at '

[01:04:38] Expected: execs

[01:04:38]     but: expected to find:

[01:04:38] [COMPILING] foo v0.5.0 (file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t11/foo)

[01:04:38] [FINISHED] release [optimized] target(s) in [..]

[01:04:38] [RUNNING] target[/]release[/]deps[/]foo-[..][EXE]

[01:04:38] thread '[..]' panicked at 'assertion failed: `(left == right)` (left: `"hello"`, right: `"nope"`)', src[/]foo.rs:14

[01:04:38] [..]

[01:04:38] 

[01:04:38] 

[01:04:38] did not find in output:

[01:04:38]    Compiling foo v0.5.0 (file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t11/foo)

[01:04:38]     Finished release [optimized] target(s) in 0.63 secs

[01:04:38]      Running target/release/deps/foo-dfc15783c07e4d52

[01:04:38] thread 'main' panicked at 'assertion failed: `(left == right)`

[01:04:38]   left: `"hello"`

[01:04:38]  right: `"nope"`', src/foo.rs:14

[01:04:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.

[01:04:38] error: bench failed

[01:04:38] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31

[01:04:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.

[01:04:38] 

[01:04:38] 

[01:04:38] failures:

[01:04:38]     cargo_bench_failing_test
