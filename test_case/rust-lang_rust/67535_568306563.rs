
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> src/lib.rs:12:1
   |
12 | impl std::ops::AddAssign for () {
   | ^^^^^-------------------^^^^^--
   | |    |                       |
   | |    |                       this is not defined in the current crate because tuples are always foreign
   | |    `std::ops::AddAssign` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead
