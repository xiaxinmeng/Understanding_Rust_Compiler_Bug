`
error[E0070]: invalid left-hand side of assignment
  --> 92979.rs:66:39
   |
66 |         &mut position[position.len()] = f32::from_be_bytes(demo(p));
   |         ----------------------------- ^
   |         |
   |         cannot assign to this expression
   |
help: you might have meant to use pattern destructuring
   |
58 |     while let position.len() < N*3 {
   |           +++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0070`.
