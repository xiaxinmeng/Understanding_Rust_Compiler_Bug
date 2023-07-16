
failures:
---- ops::RangeToInclusive_2 stdout ----
    error: inclusive range syntax is experimental (see issue #28237)
 --> <anon>:5:17
  |
5 | assert_eq!(arr[ ...2], [0, 1, 2]);
  |                 ^^^^
  |
  = help: add #![feature(inclusive_range_syntax)] to the crate attributes to enable
error: aborting due to previous error(s)
thread 'ops::RangeToInclusive_2' panicked at 'Box<Any>', src/librustc/session/mod.rs:170
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    ops::RangeToInclusive_2
test result: FAILED. 1051 passed; 1 failed; 12 ignored; 0 measured
error: test failed
