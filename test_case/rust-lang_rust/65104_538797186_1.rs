
error[E0308]: mismatched types
 --> src\main.rs:4:33
  |
4 |     s.rotate_right(n.rem_euclid(s.len()) as usize);
  |                                 ^^^^^^^ expected i8, found usize
help: you can convert an `usize` to `i8` and panic if the converted value wouldn't fit
  |
4 |     s.rotate_right(n.rem_euclid(s.len().try_into().unwrap()) as usize);
  |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
