
error: non-scalar cast: `i32` as `std::boxed::Box<std::fmt::Display>`
 --> test.rs:4:27
  |
4 |     let t: Box<Display> = 10 as Box<_>;
  |                           ^^^^^^^^^^^^

error: aborting due to previous error
