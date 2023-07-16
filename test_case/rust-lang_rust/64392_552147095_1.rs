
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
  --> src/main.rs:27:5
   |
27 |     Result::<i32, i32>::Ok(20i32)?;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
   |
   = help: the trait `std::ops::Try` is not implemented for `()`
   = note: required by `std::ops::Try::from_error`

error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == std::result::Result<i32, i32>`
  --> src/main.rs:26:43
   |
26 | async fn compile_fail_without_return() -> Result<i32, i32> {
   |                                           ^^^^^^^^^^^^^^^^ expected (), found enum `std::result::Result`
   |
   = note: expected type `()`
              found type `std::result::Result<i32, i32>`
   = note: the return type of a function must have a statically known size

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0277.
For more information about an error, try `rustc --explain E0271`.
error: could not compile `k`.
