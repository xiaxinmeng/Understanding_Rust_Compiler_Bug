
error[E0277]: cannot add `T` to `T` in const contexts
 --> src/lib.rs:6:11
  |
1 | const fn foo<T: std::ops::Add>() {
  |                 ------------- required by this bound in `foo`
...
6 |     foo::<T>()
  |           ^ no implementation for `T + T` usable in const contexts
  |
help: consider changing the `?const Add` bound for the type parameter `T` to `Add`
  |
5 | const fn bar<T: Add> (l: T, r: T) {
  |               ^^^^^
