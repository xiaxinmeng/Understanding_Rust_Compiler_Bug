
error[E0369]: binary operation `==` cannot be applied to type `&[T]`
 --> src/lib.rs:8:17
  |
8 |         if head == prefix {
  |            ---- ^^ ------ &[T]
  |            |
  |            &[T]
help: consider restricting `T` to require it implements `std::cmp::PartialEq`
  |
1 | pub fn strip_prefix<'a, T: std::cmp::PartialEq>(s: &'a [T], prefix: &[T]) -> Option<&'a [T]>
  |                          ^^^^^^^^^^^^^^^^^^^^^
