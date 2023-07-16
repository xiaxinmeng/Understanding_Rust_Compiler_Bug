rust
error[E0369]: binary operation `==` cannot be applied to type `&[T]`
 --> <source>:8:17
  |
8 |         if head == prefix {
  |            ---- ^^ ------ &[T]
  |            |
  |            &[T]
  |
  = note: an implementation of `std::cmp::PartialEq` might be missing for `&[T]`
