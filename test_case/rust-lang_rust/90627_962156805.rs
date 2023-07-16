
error[E0308]: mismatched types
 --> src/main.rs:2:18
  |
2 |     let x: i32 = Box::new(123);
  |            ---   ^^^^^^^^^^^^^ expected `i32`, found struct `Box`
  |            |
  |            expected due to this
  |
  = note: expected type `i32`
           found struct `Box<{integer}>`
help: consider dereferencing the type
  |
2 |     let x: i32 = *Box::new(123);
  |                  +
