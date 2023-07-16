
error[E0119]: conflicting implementations of trait `X`:
 --> src/lib.rs:5:1
  |
4 | impl <T : Y> X for T {}
  | ----------------------- first implementation here
5 | impl <T : Z> X for T {}
  | ^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation
