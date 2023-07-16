
error[E0277]: `Foo` doesn't implement `std::fmt::Debug`
 --> src/lib.rs:5:14
  |
5 | trait Bar<T: Debug = Foo> {}
  | -------------^^^^^-------
  | |            |
  | |            `Foo` cannot be formatted using `{:?}`
  | required by `Bar`
  |
  = help: the trait `std::fmt::Debug` is not implemented for `Foo`
  = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
