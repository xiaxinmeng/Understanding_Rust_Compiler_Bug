
error[E0277]: `()` is not an iterator
 --> src/lib.rs:6:5
  |
6 |     type IntoIter = impl Iterator<Item = Self::Item>;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` is not an iterator
  |
  = help: the trait `std::iter::Iterator` is not implemented for `()`
  = note: the return type of a function must have a statically known size

error[E0277]: `()` is not an iterator
  --> src/lib.rs:14:1
   |
14 | pub type FooIntoIter = impl Iterator<Item = ()>;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` is not an iterator
   |
   = help: the trait `std::iter::Iterator` is not implemented for `()`
   = note: the return type of a function must have a statically known size
