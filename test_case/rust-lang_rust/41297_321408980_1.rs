
$ rustc +nightly foo.rs
$ rustc +nightly bar.rs -L .
error: internal compiler error: /checkout/src/librustc_trans/collector.rs:735: Cannot create local trans-item for DefId { krate: CrateNum(12), node: DefIndex(3) => foo/8cd878b::bar[0] }

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0-nightly (215e0b10e 2017-08-08) running on x86_64-unknown-linux-gnu

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:486:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
