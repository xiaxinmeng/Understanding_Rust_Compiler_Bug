
error[E0277]: the trait bound `foo_t: std::marker::Sized` is not satisfied
  --> src/main.rs:11:21
   |
11 |         let mut a = std::ptr::null_mut();
   |                     ^^^^^^^^^^^^^^^^^^ `foo_t` does not have a constant size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `foo_t`
   = note: required by `std::ptr::null_mut`
