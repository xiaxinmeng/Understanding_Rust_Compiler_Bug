
charybdis:tmp shep$ rustc +1.18.0 what.rs
error[E0308]: mismatched types
 --> what.rs:7:28
  |
7 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
8 | |     x + 1;
9 | | }
  | |_^ expected (), found i32
  |
  = note: expected type `()`
             found type `i32`
help: consider removing this semicolon:
 --> what.rs:8:10
  |
8 |     x + 1;
  |          ^

error: aborting due to previous error
