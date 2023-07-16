rs
error: `box_syntax` has been removed
 --> test.rs:7:13
  |
7 |     let _ = box 1;
  |             ^^^^^
  |
help: use `Box::new()` instead
  |
7 |     let _ = Box::new(1);
  |             ~~~~~~~~~~~

error: `box_syntax` has been removed
 --> test.rs:8:13
  |
8 |     let _ = box T { a: 12, b: 18 };
  |             ^^^^^^^^^^^^^^^^^^^^^^
  |
help: use `Box::new()` instead
  |
8 |     let _ = Box::new(T { a: 12, b: 18 });
  |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
