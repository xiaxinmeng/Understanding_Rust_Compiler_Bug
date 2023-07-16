
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> src/main.rs:9:21
  |
9 |     pub fn new() -> Foo {
  |                     ^^^ doesn't have a size known at compile-time
  |
  = help: within `Foo`, the trait `std::marker::Sized` is not implemented for `[u8]`
  = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: required because it appears within the type `Foo`
  = note: the return type of a function must have a statically known size

error[E0061]: this function takes 1 parameter but 0 parameters were supplied
  --> src/main.rs:10:27
   |
10 |         Foo{id: 42, path: Path::new()}
   |                           ^^^^^^^^^^^ expected 1 parameter

error[E0308]: mismatched types
  --> src/main.rs:10:27
   |
10 |         Foo{id: 42, path: Path::new()}
   |                           ^^^^^^^^^^^ expected struct `std::path::Path`, found reference
   |
   = note: expected type `std::path::Path`
              found type `&std::path::Path`

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> src/main.rs:10:9
   |
10 |         Foo{id: 42, path: Path::new()}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: within `Foo`, the trait `std::marker::Sized` is not implemented for `[u8]`
   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: required because it appears within the type `Foo`
   = note: structs must have a statically known size to be initialized

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> src/main.rs:15:9
   |
15 |     let b = Foo::new();
   |         ^ doesn't have a size known at compile-time
   |
   = help: within `Foo`, the trait `std::marker::Sized` is not implemented for `[u8]`
   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: required because it appears within the type `Foo`
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> src/main.rs:15:13
   |
15 |     let b = Foo::new();
   |             ^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: within `Foo`, the trait `std::marker::Sized` is not implemented for `[u8]`
   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: required because it appears within the type `Foo`
   = note: the return type of a function must have a statically known size
