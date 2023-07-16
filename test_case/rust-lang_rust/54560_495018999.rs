
error[E0435]: attempt to use a non-constant value in a constant
 --> src/lib.rs:5:29
  |
5 |     const foo: impl Clone = x;
  |                             ^ non-constant value
