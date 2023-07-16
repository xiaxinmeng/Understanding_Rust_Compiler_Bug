
error: internal compiler error: /checkout/src/librustc_typeck/check/mod.rs:1785: no type for local variable local n (id=8)
 --> test.rs:6:26
  |
6 |         const N: usize = n;
  |                          ^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.22.0-nightly (02a24dbdd 2017-10-13) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:439:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace: (I have cut out the stacktrace for brevity)
