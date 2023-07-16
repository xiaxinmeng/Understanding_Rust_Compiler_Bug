text
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> src/lib.rs:11:6
   |
11 | impl Tr2 for [u8] {}
   |      ^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `[u8]`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: required because of the requirements on the impl of `Tr1` for `[u8]`
