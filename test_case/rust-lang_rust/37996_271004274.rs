
# rustc +nightly -V
rustc 1.16.0-nightly (6f1ae663e 2017-01-06)
# cargo +nightly build
   Compiling encoding_index_tests v0.1.5 (file:///tmp/rust-encoding/src/index/tests)
   Compiling encoding-types v0.2.0 (file:///tmp/rust-encoding/src/types)
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'assertion failed: eps.windows(2).all(|w| w[0].cmp(self, &w[1]) != Ordering::Greater)', /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-rustbuild-linux/build/src/librustc/ty/context.rs:1467
note: Run with `RUST_BACKTRACE=1` for a backtrace.

Build failed, waiting for other jobs to finish...
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'assertion failed: eps.windows(2).all(|w| w[0].cmp(self, &w[1]) != Ordering::Greater)', /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-rustbuild-linux/build/src/librustc/ty/context.rs:1467
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `encoding_index_tests`.

To learn more, run the command again with --verbose.
