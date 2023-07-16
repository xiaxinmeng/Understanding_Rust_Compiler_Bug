
error[E0507]: cannot move out of a shared reference
 --> src/main.rs:2:22
  |
2 |     let (a, mut b) = &("a".to_string(), "b".to_string());
  |             -----    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |             |
  |             data moved here
  |             move occurs because `b` has type `std::string::String`, which does not implement the `Copy` trait
