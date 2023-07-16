rust
error[E0277]: the `?` operator can only be used in an async block that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:3:33
  |
2 |       async {
  |  ___________-
3 | |         Result::<(), ()>::Ok(())?;
  | |                                 ^ cannot use the `?` operator in an async block that returns `()`
4 | |     };
  | |_____- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `FromResidual<Result<Infallible, ()>>` is not implemented for `()`
