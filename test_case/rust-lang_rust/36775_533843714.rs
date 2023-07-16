
error[E0308]: mismatched types
 --> src/main.rs:4:32
  |
4 |     let _ = Vec::with_capacity("0"); // error #1
  |                                ^^^ expected usize, found reference
  |
  = note: expected type `usize`
             found type `&'static str`

error[E0277]: `()` doesn't implement `std::fmt::Display`
 --> src/main.rs:6:15
  |
6 |     displayer(()); // error #2
  |               ^^ `()` cannot be formatted with the default formatter
...
9 | fn displayer<T: Display>(t: T) {
  | ------------------------------ required by `displayer`
  |
  = help: the trait `std::fmt::Display` is not implemented for `()`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
