
error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> src/main.rs:15:17
   |
14 |     fn f(&self) {
   |                - help: consider further restricting `Self`: `where Self: std::marker::Sized`
15 |         let _ = self as &Trait2;
   |                 ^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `Self`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: required for the cast to the object type `dyn Trait2`
