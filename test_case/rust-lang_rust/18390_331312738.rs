
error[E0308]: mismatched types
 --> src/main.rs:1:27
  |
1 |   fn add_one(x: i32) -> i32 {
  |  ___________________________^
2 | |     x + 1;
  | |          - help: consider removing this semicolon
3 | | }
  | |_^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`
