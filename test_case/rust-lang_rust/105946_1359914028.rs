
error[E0308]: mismatched types
 --> 0e1a330c6631176565a0f9d144ef565d69e68583.rs:2:9
  |
2 |     return {};
  |            ^^ expected `str`, found `()`

error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> 0e1a330c6631176565a0f9d144ef565d69e68583.rs:1:15
  |
1 | fn digit() -> str {
  |               ^^^ doesn't have a size known at compile-time
2 |     return {};
  |
  = help: the trait `Sized` is not implemented for `str`
  = note: the return type of a function must have a statically known size
