
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
 --> b.rs:3:1
  |
3 | impl a::Foo for Vec<Bar> {}
  | ^^^^^^^^^^^^^^^^--------
  | |               |
  | |               `std::vec::Vec` is not defined in the current crate
  | impl doesn't use only types from inside the current crate
  |
  = note: define and implement a trait or new type instead
