console
error[E0277]: the size for values of type `(dyn MyTrait + 'static)` cannot be known at compilation time
 --> src/main.rs:3:6
  |
3 | impl Clone for MyTrait {
  |      ^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `(dyn MyTrait + 'static)`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
