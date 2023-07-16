
error[E0191]: the value of the associated type `Item` (from the trait `std::iter::Iterator`) must be specified
 --> src/main.rs:1:25
  |
1 | pub struct Foo { i: Box<Iterator<isize>> }
  |                         ^^^^^^^^^-----^ missing associated type `Item` value, found 1 type argument instead
  |                                  |
  |                                  help: did you mean to set the associated type?: `Item = isize`

error: aborting due to previous error
