
$ cargo doc
error[E0277]: the trait bound `(): Bar` is not satisfied
 --> src/lib.rs:6:1
  |
6 | type Foo = impl Bar;
  | ^^^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `()`
  |
  = note: the return type of a function must have a statically known size
