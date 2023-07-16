
   Compiling base64 v0.10.1
   Compiling regex-automata v0.1.8
   Compiling deflate v0.7.20
   Compiling rustc-hash v1.0.1
   Compiling tokio-current-thread v0.1.6
   Compiling tokio-timer v0.2.11
thread 'rustc' panicked at 'could not initialize thread_rng: All entropy sources failed (permanently unavailable); cause: OS RNG not yet seeded (not ready yet); cause: Resource temporarily unavailable (os error 11)', /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.1/src/rngs/thread.rs:82:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (34e82a7b7 2019-09-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `deflate`.
warning: build failed, waiting for other jobs to finish...
error: failed to compile `nu v0.2.0`, intermediate artifacts can be found at `/tmp/cargo-install764Jyz`

Caused by:
  build failed

