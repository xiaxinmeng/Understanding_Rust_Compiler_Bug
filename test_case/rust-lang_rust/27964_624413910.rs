
error[E0277]: the size for values of type `(dyn Unsized + 'static)` cannot be known at compilation time
  --> src/main.rs:11:6
   |
2  | trait OopsSized<O /* missing : ?Sized */> {
   |                 - required by this bound in `OopsSized`
...
11 | impl OopsSized<Unsized> for Foo {
   |      ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn Unsized + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
