
error[E0277]: the trait bound `foo_t: std::marker::Sized` is not satisfied
  --> src/main.rs:10:9
   |
10 |     let mut a = std::mem::uninitialized();
   |         ^^^^^ `foo_t` does not have a constant size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `foo_t`
   = note: all local variables must have a statically known size
