
error[E0277]: the size for values of type `(dyn std::iter::Iterator<Item=u32> + 'static)` cannot be known at compilation time
 --> src/lib.rs:1:13
  |
1 | fn foo() -> Iterator<Item = u32> { panic!() }
  |             ^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `(dyn std::iter::Iterator<Item=u32> + 'static)`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: the return type of a function must have a statically known size
