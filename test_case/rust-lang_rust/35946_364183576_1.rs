
error[E0277]: the `?` operator can only be used in a function that returns `Result` (or another type that implements `std::ops::Try`)
 --> src/main.rs:2:5
  |
2 |     std::fs::File::open("foo")?;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`

error[E0308]: mismatched types
 --> src/main.rs:3:5
  |
3 |     Ok(())
  |     ^^^^^^ expected (), found enum `std::result::Result`
  |
  = note: expected type `()`
             found type `std::result::Result<(), _>`
help: try adding a semicolon
  |
3 |     Ok(());
  |           ^
help: try adding a return type
  |
1 | fn foo() -> std::result::Result<(), _> {
  |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
