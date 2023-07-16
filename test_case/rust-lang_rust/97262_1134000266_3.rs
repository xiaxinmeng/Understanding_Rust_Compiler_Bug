
ekuecks@ekuecks-mbp test_diag % rustc +stage1 main.rs
error[E0282]: type annotations needed for the closure `fn(&_) -> i32`
 --> main.rs:2:13
  |
2 |     let a = |a: &_| -> i32 { 1 };
  |             ^^^^^-^^^^^^^^^^^^^^
  |             |    |
  |             |    consider giving this closure parameter an explicit type without `_` placeholders
  |             cannot infer type for closure `[closure@main.rs:2:13: 2:33]`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
