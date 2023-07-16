
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
 --> src/main.rs:5:1
  |
5 | impl<T: LocalMarker> fmt::Display for Vec<T> { } 
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------
  | |                                     |
  | |                                     `std::vec::Vec` is not defined in the current crate
  | impl doesn't use only types from inside the current crate
  |
  = note: define and implement a trait or new type instead
