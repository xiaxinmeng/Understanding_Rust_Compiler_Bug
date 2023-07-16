
error[E0317]: if may be missing an else clause
 --> test.rs:2:5
  |
2 | /     if bar % 5 == 0 {
3 | |         return 1;
4 | |     }
  | |_____^ expected (), found usize
  |
  = note: expected type `()`
             found type `usize`

error: aborting due to previous error(s)
