
Execution
Close
Standard Error
   Compiling playground v0.0.1 (/playground)
error[E0277]: the size for values of type `Self` cannot be known at compilation time
 --> src/main.rs:5:12
  |
5 |     fn foo(self) -> Box<dyn Any + 'a> {
  |            ^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `Self`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: all local variables must have a statically known size
  = help: unsized locals are gated as an unstable feature
help: consider further restricting `Self`
  |
5 |     fn foo(self) -> Box<dyn Any + 'a> where Self: std::marker::Sized {
  |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the size for values of type `Self` cannot be known at compilation time
 --> src/main.rs:6:18
  |
6 |         Box::new(self)
  |                  ^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `Self`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: required by `std::boxed::Box::<T>::new`
help: consider further restricting `Self`
  |
5 |     fn foo(self) -> Box<dyn Any + 'a> where Self: std::marker::Sized {
  |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the size for values of type `Self` cannot be known at compilation time
 --> src/main.rs:6:9
  |
6 |         Box::new(self)
  |         ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `Self`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: required for the cast to the object type `dyn std::any::Any`
help: consider further restricting `Self`
  |
5 |     fn foo(self) -> Box<dyn Any + 'a> where Self: std::marker::Sized {
  |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the size for values of type `Self` cannot be known at compilation time
 --> src/main.rs:6:9
  |
6 |         Box::new(self)
  |         ^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `Self`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: all function arguments must have a statically known size
  = help: unsized locals are gated as an unstable feature
help: consider further restricting `Self`
  |
5 |     fn foo(self) -> Box<dyn Any + 'a> where Self: std::marker::Sized {
  |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `foo` found for reference `&'static str` in the current scope
  --> src/main.rs:11:36
   |
11 |     let _foo: Box<dyn Any> = "foo".foo();
   |                                    ^^^ method not found in `&'static str`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Foobar` defines an item `foo`, perhaps you need to implement it
  --> src/main.rs:4:1
   |
4  | trait Foobar<'a> {
   | ^^^^^^^^^^^^^^^^

error: aborting due to 5 previous errors
