
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): a value of type `Vec<X>` cannot be built from an iterator over elements of type `&X`
 --> src/main.rs:6:7
  |
6 |     i.collect()
  |       ^^^^^^^ value of type `Vec<X>` cannot be built from `std::iter::Iterator<Item=&X>`
  |
  = help: the trait `FromIterator<&X>` is not implemented for `Vec<X>`
  = help: the trait `FromIterator<T>` is implemented for `Vec<T>`
note: required by a bound in `collect`

For more information about this error, try `rustc --explain E0277`.
