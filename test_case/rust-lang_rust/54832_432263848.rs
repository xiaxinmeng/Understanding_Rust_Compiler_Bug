
error[E0017]: references in constants may only refer to immutable values
 --> src/main.rs:5:7
  |
5 |     { &mut x; } // Delete `mut` and this will work.
  |       ^^^^^^ constants require immutable values

error: aborting due to previous error
