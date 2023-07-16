rust
error[E0308]: mismatched types
 --> test3.rs:2:5
  |
2 |     Ok(42)?
  |     ^^^^^^^ expected i32, found enum `std::result::Result`
  |
  = note: expected type `i32`
  = note:    found type `std::result::Result<_, _>`

error: aborting due to previous error
