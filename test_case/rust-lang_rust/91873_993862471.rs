
error[E0277]: expected a `FnMut<(char,)>` closure, found `u8`
 --> f23.rs:2:20
  |
2 |     s.strip_suffix(b'\n').unwrap_or(s)
  |       ------------ ^^^^^ expected an `FnMut<(char,)>` closure, found `u8`
  |       |
  |       required by a bound introduced by this call
  |
  = help: the trait `FnMut<(char,)>` is not implemented for `u8`
