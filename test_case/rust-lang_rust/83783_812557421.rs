
error[E0308]: mismatched types
 --> src/lib.rs:5:31
  |
5 |     consume_reference::<i32>(&async { Box::new(7_i32) }.await);
  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |                               |
  |                               expected `i32`, found struct `Box`
  |                               help: consider dereferencing the type: `*async { Box::new(7_i32) }.await`
  |
  = note: expected type `i32`
           found struct `Box<i32>`

error: aborting due to previous error
