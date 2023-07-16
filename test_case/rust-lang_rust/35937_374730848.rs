
error[E0594]: cannot assign to field `s.x` of immutable binding
 --> src/main.rs:4:1
  |
3 | let s = S { x: 42 };
  |     - consider changing this to `mut s`
4 | s.x += 1;
  | ^^^^^^^^ cannot mutably borrow field of immutable binding
