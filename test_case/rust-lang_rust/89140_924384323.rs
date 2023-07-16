rust
error[E0782]: trait objects must include the `dyn` keyword
 --> src/lib.rs:5:15
  |
5 | impl<T> Trait<MyInto<T>> for Struct {}
  |               ^^^^^^^^^
  |
help: add `dyn` keyword before this trait
  |
5 | impl<T> Trait<dyn MyInto<T>> for Struct {}
  |               +++
