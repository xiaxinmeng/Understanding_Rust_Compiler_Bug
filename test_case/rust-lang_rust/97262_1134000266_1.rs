
ekuecks@ekuecks-mbp test_diag % rustc +stage1 main.rs
error[E0282]: type annotations needed
 --> main.rs:3:14
  |
3 |     let b = |a: _| -> i32 { 1 };
  |              ^ consider giving this closure parameter a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
