
error[E0277]: cannot subtract `T` from `usize`
 --> test4.rs:3:38
  |
3 |     (x << k as T) | (x >> (bit_count - k as T))
  |                                      ^ no implementation for `usize - T`
  |
  = help: the trait `Sub<T>` is not implemented for `usize`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
  |
1 | fn rot<T : std::ops::Shl<Output = T>>(x:&T, k:u32) -> T where usize: Sub<T> {
  |                                                         +++++++++++++++++++


error[E0369]: no implementation for `&T >> _`
 --> test4.rs:3:24
  |
3 |     (x << k as T) | (x >> (bit_count - k as T))
  |                      - ^^ -------------------- _
  |                      |
  |                      &T
  |
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
  |
1 | fn rot<T : std::ops::Shl<Output = T>>(x:&T, k:u32) -> T where &T: Shr<_> {
  |
