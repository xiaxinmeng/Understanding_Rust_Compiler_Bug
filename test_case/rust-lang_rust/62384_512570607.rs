
error[E0308]: mismatched types
 --> src/test/ui/suggestions/missing_return_type.rs:7:5
  |
3 | fn main() {
  |           - expected `()` because of default return type
...
7 |     Ok(Err(())?)
  |     ^^^^^^^^^^^^- help: try adding a semicolon: `;`
  |     |
  |     expected (), found enum `std::result::Result`
  |
  = note: expected type `()`
             found type `std::result::Result<_, _>`
