
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: mutable allocation in constant
 --> src/lib.rs:3:36
  |
3 | pub const FOO:  &'static SyncPtr = &SyncPtr(&BAR as _);
  |                                    ^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:349:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic
