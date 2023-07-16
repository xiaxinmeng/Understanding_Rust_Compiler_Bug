
error: internal compiler error: librustc_mir/transform/qualify_consts.rs:273: multiple assignments to _1
 --> src/main.rs:1:24
  |
1 | const ARR: [(); 0] = [ 'a: while break 'a {}; 0];
  |                        ^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:488:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0 (a77568041 2018-05-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.
