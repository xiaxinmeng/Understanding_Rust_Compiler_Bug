
error[E0277]: the trait bound `i32: std::process::Termination` is not satisfied
  --> src/bin/termination-trait-main-i32.rs:11:14
   |
11 | fn main() -> i32 {
   |              ^^^ `main` can only return types that implement std::process::Termination, not `i32`
   |
   = help: consider using `()`, or a `Result`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
