
error[E0308]: mismatched types
 --> test.rs:2:5
  |
2 | /     for x in &[0, 10] {
3 | |         return 1;
4 | |     }
  | |_____^ expected usize, found ()
  |
  = note: expected type `usize`
             found type `()`

error: aborting due to previous error(s)
