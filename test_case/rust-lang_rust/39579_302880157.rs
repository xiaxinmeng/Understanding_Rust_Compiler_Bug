
error[E0369]: binary operation `+` cannot be applied to type `()`
 --> test.rs:2:5
  |
2 |     () + 10i32;
  |     ^^^^^^^^^^
  |
  = note: an implementation of `std::ops::Add` might be missing for `()`

error[E0277]: the trait bound `i32: std::ops::Add<()>` is not satisfied
 --> test.rs:3:5
  |
3 |     10i32 + ();
  |     ^^^^^^^^^^ the trait `std::ops::Add<()>` is not implemented for `i32`
  |
  = note: no implementation for `i32 + ()`
