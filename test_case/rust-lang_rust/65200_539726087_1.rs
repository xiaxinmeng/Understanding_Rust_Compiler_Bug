
error[E0277]: the size for values of type `Self` cannot be known at compilation time
 --> src/lib.rs:3:1
  |
3 | trait Trait: Super<Self> {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `Self`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = help: consider adding a `where Self: std::marker::Sized` bound
note: required by `Super`
 --> src/lib.rs:1:1
  |
1 | trait Super<A> {}
  | ^^^^^^^^^^^^^^
