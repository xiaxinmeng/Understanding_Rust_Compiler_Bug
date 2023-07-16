
error[E0277]: a collection of type `std::vec::Vec<X>` cannot be built from an iterator over elements of type `&X`
 --> src/main.rs:6:7
  |
6 |     i.collect()
  |       ^^^^^^^ a collection of type `std::vec::Vec<X>` cannot be built from `std::iter::Iterator<Item=&X>`
  |
  = help: the trait `std::iter::FromIterator<&X>` is not implemented for `std::vec::Vec<X>`
