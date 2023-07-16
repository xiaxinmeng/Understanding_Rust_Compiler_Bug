
error[E0277]: the trait bound `u32: T` is not satisfied
 --> src/main.rs:6:5
  |
6 |     function(0u32);
  |     ^^^^^^^^ the trait `T` is not implemented for `u32`
  |
note: required by `function`
 --> src/main.rs:3:1
  |
3 | fn function<U: T>(u: U) {}
  | ^^^^^^^^^^^^^^^^^^^^^^^
