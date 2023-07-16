
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:3:19
  |
3 |     const Y: u8 = x;
  |                   ^ did you mean `Y`?
  |
  | note: the variable `x` defined in the surrounding fn is not in scope within a nested `const`
