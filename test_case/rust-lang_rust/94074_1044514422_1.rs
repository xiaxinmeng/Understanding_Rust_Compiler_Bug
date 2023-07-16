rust
error[E0308]: mismatched types
 --> src/main.rs:4:9
  |
4 |     foo((0));
  |         ^^^ expected enum `Option`, found integer
  |
  = note: expected enum `Option<i32>`
             found type `{integer}`
help: try wrapping the expression in `Some`
  |
4 |     foo(Some((0)));
  |         +++++   +
