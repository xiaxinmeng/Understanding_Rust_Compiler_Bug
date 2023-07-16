
error[E0425]: cannot find value `E` in this scope
 --> test.rs:2:17
  |
2 |     let x = [0; E];
  |                 ^ did you mean `f`?
  |
help: possible candidates are found in other modules, you can import them into scope
  | use std::f32::consts::E;
  | use std::f64::consts::E;

error: aborting due to previous error(s)
