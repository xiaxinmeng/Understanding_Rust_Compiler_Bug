
...   
Compiling isatty v0.1.9
   Compiling directories v1.0.2
   Compiling subprocess v0.1.18
   Compiling want v0.2.0
   Compiling render-tree v0.1.1
thread 'rustc' panicked at 'could not initialize thread_rng: All entropy sources failed (permanently unavailable); cause: OS RNG not yet seeded (not ready yet); cause: Resource temporarily unavailable (os error 11)', /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.1/src/rngs/thread.rs:82:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0 (eae3437df 2019-08-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `want`.
warning: build failed, waiting for other jobs to finish...
error: failed to compile `nu v0.2.0`, intermediate artifacts can be found at `/tmp/cargo-installJ61Ab7`

Caused by:
  build failed
