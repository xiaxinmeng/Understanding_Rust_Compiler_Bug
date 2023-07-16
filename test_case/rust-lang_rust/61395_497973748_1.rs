
error[E0308]: mismatched types
 --> src/main.rs:9:9
  |
9 |     foo(x);
  |         ^ expected usize, found struct `Generic`
  |
  = note: expected type `usize`
             found type `Generic<0usize>`
