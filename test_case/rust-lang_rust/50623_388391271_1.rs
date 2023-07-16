
error[E0308]: mismatched types
 --> src/main.rs:1:19
  |
1 | static U: usize = |x: u8| {};
  |                   ^^^^^^^^^^ expected usize, found closure
  |
  = note: expected type `usize`
             found type `[closure@src/main.rs:1:19: 1:29]`
