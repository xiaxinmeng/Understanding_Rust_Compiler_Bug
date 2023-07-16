
error[E0308]: mismatched types
 --> src/main.rs:5:19
  |
5 |   fn bar() -> usize {
  |   ----------------- this function doesn't return anything
6 |     3;
  |      - help: consider removing this semicolon
  |
  = note: expected type `usize`
             found type `()`
