
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:9:30
  |
8 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
9 |     let x = retoptionres("4")??;
  |                              ^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `FromResidual<Result<Infallible, ParseIntError>>` is not implemented for `()`

error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:9:31
  |
8 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
9 |     let x = retoptionres("4")??;
  |                               ^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `()`
