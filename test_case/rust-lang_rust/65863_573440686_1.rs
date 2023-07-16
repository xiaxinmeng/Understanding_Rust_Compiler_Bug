
error[E0277]: `()` is not an iterator
 --> src/lib.rs:3:1
  |
3 | type Test = impl Iterator<Item = ()>;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` is not an iterator
  |
  = help: the trait `std::iter::Iterator` is not implemented for `()`
  = note: the return type of a function must have a statically known size
