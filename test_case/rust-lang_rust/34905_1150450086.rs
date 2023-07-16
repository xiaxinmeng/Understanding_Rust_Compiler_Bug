
error[E0308]: mismatched types
 --> src/main.rs:5:32
  |
5 |     let x: Option<Option<_>> = y;
  |            -----------------   ^ expected enum `Option`, found enum `Result`
  |            |
  |            expected due to this
  |
  = note: expected enum `Option<Option<_>>`
             found enum `Option<Result<_, ()>>`
help: try wrapping the expression in `Some`
  |
5 |     let x: Option<Option<_>> = Some(y);
  |                                +++++ +
