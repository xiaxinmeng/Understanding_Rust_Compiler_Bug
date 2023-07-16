
error[E0277]: the size for values of type `Self` cannot be known at compilation time
 --> src/lib.rs:3:17
  |
3 |         let _ = self as &Trait2;
  |                 ^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `Self`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: required for the cast to the object type `dyn Trait2`
help: consider further restricting `Self`
  |
2 |     fn f(&self) where Self: std::marker::Sized {
  |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
