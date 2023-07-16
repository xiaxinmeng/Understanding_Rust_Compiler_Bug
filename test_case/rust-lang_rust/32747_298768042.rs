
warning: unnecessary `unsafe` block
 --> test.rs:4:9
  |
4 |         unsafe { $e }
  |         ^^^^^^^^^^^^^ unnecessary `unsafe` block
...
9 |     let x : usize = unsafe { another_unsafe!(std::mem::transmute(3usize)) };
  |                              -------------------------------------------- in this macro invocation
  |
  = note: #[warn(unused_unsafe)] on by default
note: because it's nested under this `unsafe` block
 --> test.rs:9:21
  |
9 |     let x : usize = unsafe { another_unsafe!(std::mem::transmute(3usize)) };
  |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

