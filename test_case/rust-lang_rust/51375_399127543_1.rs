
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:3:19
  |
2 |     let x: u32 = 22;
  |         - note: this `x` is not in scope within the nested `const`
3 |     const Y: u8 = x;
  |                   ^ did you mean `Y`?
