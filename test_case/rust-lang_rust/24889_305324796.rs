
error[E0308]: mismatched types
 --> test.rs:1:29
  |
1 |   fn foo() -> Result<u8, u64> {
  |  _____________________________^
2 | |     Ok(1);
3 | | }
  | |_^ expected enum `std::result::Result`, found ()
  |
  = note: expected type `std::result::Result<u8, u64>`
             found type `()`
help: consider removing this semicolon:
 --> test.rs:2:10
  |
2 |     Ok(1);
  |          ^

error: aborting due to previous error(s)
