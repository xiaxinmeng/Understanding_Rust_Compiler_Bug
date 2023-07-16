
thread 'rustc' panicked at 'use-after-free in `proc_macro` handle', /rustc/433aae93e4ef866a1fdfefad136b32ed89acd3e7/src/libproc_macro/bridge/handle.rs:42:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (433aae93e 2020-02-14) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden
