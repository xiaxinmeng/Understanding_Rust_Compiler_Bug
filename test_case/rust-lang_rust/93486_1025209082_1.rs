
error[E0070]: invalid left-hand side of assignment
 --> src/main.rs:7:31
  |
7 |         v.last_mut().unwrap() = 3_u8;
  |         --------------------- ^
  |         |
  |         cannot assign to this expression
  |
help: you might have meant to use pattern destructuring
  |
6 |     while let let Some(_) = iter.next() {
  |           +++

For more information about this error, try `rustc --explain E0070`.
