
   Compiling playground v0.0.1 (file:///playground)
error: `derive` can be only be applied to items
  --> src/main.rs:10:10
   |
10 | #[derive(Debug)]
   |          ^^^^^

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.22.0-nightly (744dd6c1d 2017-09-02) running on x86_64-unknown-linux-gnu

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335:20
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
