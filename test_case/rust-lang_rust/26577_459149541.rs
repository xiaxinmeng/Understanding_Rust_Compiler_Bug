
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 |       let the_sum :i32 = {
  |  ________________________^
8 | |         foo();
  | |              - help: consider removing this semicolon
9 | |     };
  | |_____^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`

error: aborting due to previous error
