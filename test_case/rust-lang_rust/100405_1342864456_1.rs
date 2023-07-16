
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
 --> src/main.rs:3:6
  |
3 | impl<T, U: Foo<T>> Drop for U {
  |      ^ unconstrained type parameter

For more information about this error, try `rustc --explain E0207`.
