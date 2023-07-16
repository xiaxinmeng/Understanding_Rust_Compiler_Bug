
error[E0308]: mismatched types
 --> src/main.rs:3:16
  |
3 |         return Ok(6);
  |                ^^^^^ expected i32, found enum `std::result::Result`
  |
  = note: expected type `i32`
             found type `std::result::Result<{integer}, _>`
