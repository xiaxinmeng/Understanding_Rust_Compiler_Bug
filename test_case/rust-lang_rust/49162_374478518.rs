
error[E0277]: the trait bound `i32: std::process::Termination` is not satisfied
  --> src/bin/termination-trait-main-i32.rs:11:18
   |
11 |   fn main() -> i32 {
   |  __________________^
12 | | //~^ ERROR the trait bound `i32: std::process::Termination` is not satisfied [E0277]
13 | |     0
14 | | }
   | |_^ `main` can only return types that implement std::process::Termination, not `i32`
   |
   = help: the trait `std::process::Termination` is not implemented for `i32`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
