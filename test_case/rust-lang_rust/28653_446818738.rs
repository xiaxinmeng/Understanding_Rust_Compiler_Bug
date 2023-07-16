
error[E0277]: the size for values of type `(dyn Foo + 'static)` cannot be known at compilation time
 --> src/lib.rs:2:8
  |
2 | fn foo(arg: Foo) {} //Foo is a trait
  |        ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `(dyn Foo + 'static)`
  = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: all local variables must have a statically known size
  = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
 --> src/lib.rs:3:9
  |
3 | fn bar (arg: [i32]) {}
  |         ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `[i32]`
  = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: all local variables must have a statically known size
  = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> src/lib.rs:4:9
  |
4 | fn quz (arg: str) {}
  |         ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `str`
  = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: all local variables must have a statically known size
  = help: unsized locals are gated as an unstable feature
