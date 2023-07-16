
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> src/main.rs:4:5
  |
1 | fn sized_ref<T>(_: &T, _: &T) {}
  |              - required by this bound in `sized_ref`
...
4 |     sized_ref("First", "Second");
  |     ^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
help: consider relaxing the implicit `Sized` restriction
  |
1 | fn sized_ref<T: ?Sized>(_: &T, _: &T) {}
  |               ^^^^^^^^
