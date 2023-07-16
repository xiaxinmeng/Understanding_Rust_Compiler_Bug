
cargo build
   Compiling pyo3 v0.4.1
error: internal compiler error: librustc/traits/specialize/mod.rs:104: When translating substitutions for specialization, the expected specialization failed to hold

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:600:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (6b9b97bd9 2018-11-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `pyo3`.

To learn more, run the command again with --verbose.
