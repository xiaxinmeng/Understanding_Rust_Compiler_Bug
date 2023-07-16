
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> src/lib.rs:12:1
   |
12 | impl std::ops::AddAssign<Rhs=()> for () {
   | ^^^^^---------------------------^^^^^--
   | |    |                               |
   | |    |                               this is not defined in the current crate because tuples are always foreign
   | |    this is not defined in the current crate because tuples are always foreign
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead
