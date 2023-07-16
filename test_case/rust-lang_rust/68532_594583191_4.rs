
error[E0277]: the size for values of type `T` cannot be known at compilation time
 --> src/main.rs:9:21
  |
3 | pub struct A<T>(T);
  | ------------------- required by `A`
4 | 
5 | impl<T> A<T> {
  |      - this type parameter needs to be `std::marker::Sized`
...
9 |         let b = [0; Self::N];
  |                     ^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `T`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
