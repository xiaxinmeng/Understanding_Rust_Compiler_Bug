
error[E0308]: mismatched types
 --> src/lib.rs:3:5
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
