
error[E0435]: attempt to use a non-constant value in a constant
 --> t2.rs:3:19
  |
2 |     let   length: usize = 2;
  |       ---------- help: consider using `const` instead of `let`: `const length`
3 |     let arr = [0; length];
  |                   ^^^^^^ non-constant value
