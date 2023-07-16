
error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
 --> ../rust0/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error.rs:8:5
  |
8 |     [Foo(String::new()); 4];
  |     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `std::string::String`
  |
  = note: required because of the requirements on the impl of `std::marker::Copy` for `Foo<std::string::String>`
  = note: the `Copy` trait is required because the repeated element will be copied

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
