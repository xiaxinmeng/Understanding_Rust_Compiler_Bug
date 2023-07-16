

[01:15:04] ---- concurrent_installs stdout ----

[01:15:04] 	thread 'concurrent_installs' panicked at '

[01:15:04] Expected: execs

[01:15:04]     but: exited with exit code: 101

[01:15:04] --- stdout

[01:15:04] 

[01:15:04] --- stderr

[01:15:04]     Updating registry `file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t0/registry`

[01:15:04]     Blocking waiting for file lock on the registry index

[01:15:04] error: could not find `foo` in `registry https://github.com/rust-lang/crates.io-index`

[01:15:04] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31

[01:15:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.

[01:15:04] 

[01:15:04] 

[01:15:04] failures:

[01:15:04]     concurrent_installs
