
error[E0277]: the trait bound `Self: std::ops::Termination` is not satisfied
  --> src/rustc/rustc.rs:15:11
   |
15 | fn main() { rustc_driver::main() }
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::ops::Termination` is not implemented for `Self`
   |
   = help: consider adding a `where Self: std::ops::Termination` bound
