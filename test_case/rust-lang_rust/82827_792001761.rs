
error: `let` expressions are not supported here
 --> src/main.rs:3:7
  |
3 |   if (let Some(y) = x) {
  |       ^^^^^^^^^^^^^^^
  |
  = note: only supported directly in conditions of `if`- and `while`-expressions
  = note: as well as when nested within `&&` and parenthesis in those conditions
