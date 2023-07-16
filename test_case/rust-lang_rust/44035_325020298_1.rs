
charybdis:tmp shep$ rustc +stable what.rs
error[E0308]: mismatched types
 --> what.rs:7:28
  |
7 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
8 | |     x + 1;
9 | | }
  | |_^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`
help: consider removing this semicolon:
 --> what.rs:8:10
  |
8 |     x + 1;
  |          ^
