
error[E0277]: the trait bound `foo_t: std::marker::Sized` is not satisfied
  --> src/main.rs:15:5
   |
15 |     std::ptr::null()
   |     ^^^^^^^^^^^^^^ `foo_t` does not have a constant size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `foo_t`
   = note: required by `std::ptr::null`
