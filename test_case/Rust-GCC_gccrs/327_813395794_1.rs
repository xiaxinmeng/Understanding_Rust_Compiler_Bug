
error[E0308]: mismatched types
 --> 1.rs:3:16
  |
3 |     let y:i8 = x;
  |           --   ^ expected `i8`, found `i32`
  |           |
  |           expected due to this
  |
help: you can convert an `i32` to an `i8` and panic if the converted value doesn't fit
  |
3 |     let y:i8 = x.try_into().unwrap();
  |                ^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
 --> 1.rs:4:5
  |
1 | fn foo() -> i32 {
  |             --- expected `i32` because of return type
...
4 |     y
  |     ^
  |     |
  |     expected `i32`, found `i8`
  |     help: you can convert an `i8` to an `i32`: `y.into()`
