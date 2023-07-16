
error[E0277]: `()` doesn't implement `std::fmt::Display`
 --> /home/gh-compiler-errors/test.rs:8:17
  |
8 |     fn bar() -> () {}
  |                 ^^ `()` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `()`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `Foo::bar::{opaque#0}`
 --> /home/gh-compiler-errors/test.rs:4:22
  |
4 |     fn bar() -> impl std::fmt::Display;
  |                      ^^^^^^^^^^^^^^^^^ required by this bound in `Foo::bar::{opaque#0}
