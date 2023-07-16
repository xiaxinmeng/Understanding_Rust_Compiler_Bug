
~/Downloads $ rustc test.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'assertion failed: index.end <= self.len()', /Users/rustbuild/src/rust-buildbot/slave/beta-dist-rustc-mac/build/src/libcore/slice.rs:519


~/Downloads $ rustc --version
rustc 1.0.0-beta.2 (e9080ec39 2015-04-16) (built 2015-04-16)
