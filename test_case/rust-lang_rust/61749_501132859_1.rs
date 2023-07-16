rust
error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
 --> src/main.rs:4:5
  |
4 |     [Foo(String::new()); 4];
  |     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `std::string::String`
  |
  = note: required because of the requirements on the impl of `std::marker::Copy` for `Foo<std::string::String>`
  = note: the `Copy` trait is required because the repeated element will be copied
