rust
error[E0277]: the trait bound `C: std::clone::Clone` is not satisfied
  --> src\main.rs:28:23
   |
28 |     cloner.call_clone(&inner);
   |                       ^^^^^^ the trait `std::clone::Clone` is not implemented for `C`
   |
   = note: required because of the requirements on the impl of `std::clone::Clone` for `Inner<C>`
help: consider restricting type parameter `C`
   |
21 | fn clear_error<C: std::clone::Clone>() {
   |                 ^^^^^^^^^^^^^^^^^^^
