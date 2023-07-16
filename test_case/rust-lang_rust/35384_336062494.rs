
error[E0277]: the trait bound `std::option::Option<std::ptr::Shared<_>>: std::fmt::Pointer` is not satisfied
  --> main.rs:13:52
   |
13 |     assert_eq!(format!("{:p}", a), format!("{:p}", x));
   |                                                    ^ the trait `std::fmt::Pointer` is not implemented for `std::option::Option<std::ptr::Shared<_>>`
   |
   = note: required by `std::fmt::Pointer::fmt`

