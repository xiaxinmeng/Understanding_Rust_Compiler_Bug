
failures:

---- f32::f32::atan2_0 stdout ----
    error: no associated item named `EPSILON` found for type `f32` in the current scope
  --> <anon>:16:29
   |
16 | assert!(abs_difference_1 <= f32::EPSILON);
   |                             ^^^^^^^^^^^^

error: no associated item named `EPSILON` found for type `f32` in the current scope
  --> <anon>:17:29
   |
17 | assert!(abs_difference_2 <= f32::EPSILON);
   |                             ^^^^^^^^^^^^

error: aborting due to previous error(s)

thread 'f32::f32::atan2_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:171
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    f32::f32::atan2_0

test result: FAILED. 545 passed; 1 failed; 19 ignored; 0 measured

error: test failed
