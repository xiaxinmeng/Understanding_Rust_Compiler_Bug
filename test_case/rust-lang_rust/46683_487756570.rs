
error[E0277]: the size for values of type `dyn Something` cannot be known at compilation time
  --> src/main.rs:10:5
   |
10 |     foo(x);
   |     ^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `dyn Something`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
note: required by `foo`
  --> src/main.rs:6:1
   |
6  | fn foo<T: Something>(_: &T) { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
