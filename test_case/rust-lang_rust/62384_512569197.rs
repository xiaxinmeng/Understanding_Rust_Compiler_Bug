
error[E0308]: mismatched types
 --> src/test/ui/suggestions/missing_return_type.rs:6:5
  |
6 |     Result::<(), ()>::Ok(())
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found enum `std::result::Result`
  |
  = note: expected type `()`
             found type `std::result::Result<(), ()>`
help: try adding a semicolon
  |
6 |     Result::<(), ()>::Ok(());
  |                             ^
help: try adding a return type
  |
3 | fn main() -> std::result::Result<(), ()> {
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
