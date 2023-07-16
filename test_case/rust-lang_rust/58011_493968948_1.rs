
error[E0277]: the trait bound `(): A` is not satisfied
 --> test.rs:6:1
  |
6 | existential type T: A;
  | ^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `()`
  |
  = note: the return type of a function must have a statically known size
