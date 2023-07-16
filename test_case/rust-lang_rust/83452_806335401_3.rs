
error[E0277]: cannot add `T` to `T`
 --> src/lib.rs:6:11
  |
1 | fn foo<T: std::ops::Add>() {
  |           ------------- required by this bound in `foo`
...
6 |     foo::<T>()
  |           ^ no implementation for `T + T`
  |
help: consider restricting type parameter `T`
  |
5 | fn bar<T: Add> (l: T, r: T) {
  |         ^^^^^
