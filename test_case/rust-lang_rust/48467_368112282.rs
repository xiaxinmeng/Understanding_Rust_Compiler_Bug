
error[E0XXX]: ambiguous lifetime for `impl Iterator`
 --> src/main.rs:5:29
  |
5 | fn a<T>(items: &[T]) -> Box<impl Iterator> {
  |                             ^^^^^^^^^^^^^ this trait bound implies lifetime `'static`
help: add a lifetime binding to remove the `'static` requirement
  |
5 | fn a<T>(items: &[T]) -> Box<impl Iterator + '_> {
  |                             ^^^^^^^^^^^^^^^^^^
help: otherwise, change the type of `items` to `&'static [T]` to satisfy the `'static` requirement
  |
5 | fn a<T>(items: &'static [T]) -> Box<impl Iterator> {
  |                ^^^^^^^^^^^^

