rust
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: mutable allocation in constant
 --> src/lib.rs:1:39
  |
1 | pub const FOO:  &'static *const u32 = &(&BAR as _);
  |                                       ^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:349:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (a9c1c04e9 2019-12-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: could not compile `playground`.

To learn more, run the command again with --verbose.

