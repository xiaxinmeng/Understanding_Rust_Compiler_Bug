
error: internal compiler error: broken MIR ((_5.0: &'static [u8; 0])): bad field access (&[u8]: &'static [u8; 0]): Sorts(ExpectedFound { expected: [u8; 0], found: [u8] })
 --> test.rs:5:10
  |
5 |         (B, 0)  => {},
  |          ^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:421
note: Run with `RUST_BACKTRACE=1` for a backtrace.
