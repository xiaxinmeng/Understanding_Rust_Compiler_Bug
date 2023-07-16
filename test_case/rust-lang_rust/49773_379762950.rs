   Compiling serde v1.0.37
error: incremental compilation: could not create session directory lock file: No such process (os error 3)

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.24.1 running on sparc64-unknown-linux-gnu

thread 'rustc' panicked at 'src/librustc/session/mod.rs:665: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/session/mod.rs:1141:26
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `build_helper`.
warning: build failed, waiting for other jobs to finish...
