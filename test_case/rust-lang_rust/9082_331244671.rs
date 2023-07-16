
error[E0277]: the trait bound `std::vec::Vec<X>: std::iter::FromIterator<&X>` is not satisfied
 --> src/main.rs:6:7
  |
6 |     i.collect()
  |       ^^^^^^^ a collection of type `std::vec::Vec<X>` cannot be built from an iterator over elements of type `&X`
  |
  = help: the trait `std::iter::FromIterator<&X>` is not implemented for `std::vec::Vec<X>`
