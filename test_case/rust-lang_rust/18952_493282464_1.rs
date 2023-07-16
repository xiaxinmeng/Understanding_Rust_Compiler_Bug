
error[E0277]: expected a `std::ops::FnMut<(isize, isize)>` closure, found `Foo`
 --> src/main.rs:8:6
  |
8 | impl Fn<(isize, isize)> for Foo {
  |      ^^^^^^^^^^^^^^^^^^ expected an `FnMut<(isize, isize)>` closure, found `Foo`
  |
  = help: the trait `std::ops::FnMut<(isize, isize)>` is not implemented for `Foo`
