txt
error[E0119]: conflicting implementations of trait `std::marker::Send` for type `Foo`:
 --> src/main.rs:4:1
  |
3 | unsafe impl Send for Foo {}
  | ------------------------ first implementation here
4 | unsafe impl Send for Foo {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Foo`
