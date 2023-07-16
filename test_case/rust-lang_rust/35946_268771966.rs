rust
fn does_not_return_result() -> i32 {
    Ok(42)?
}

error[E0308]: mismatched types
 --> test3.rs:2:5
  |
2 |     Ok(42)?;
  |     ^^^^^^^ expected i32, found enum `std::result::Result`
  |
  = note: expected type `i32`
  = note:    found type `std::result::Result<_, _>`

error[E0308]: mismatched types
 --> test3.rs:1:36
  |
1 |   fn does_not_return_result() -> i32 {
  |  ____________________________________^ starting here...
2 | |     Ok(42)?;
3 | | }
  | |_^ ...ending here: expected i32, found ()
  |
  = note: expected type `i32`
  = note:    found type `()`
help: consider removing this semicolon:
 --> test3.rs:2:12
  |
2 |     Ok(42)?;
  |            ^

error: aborting due to 2 previous errors

