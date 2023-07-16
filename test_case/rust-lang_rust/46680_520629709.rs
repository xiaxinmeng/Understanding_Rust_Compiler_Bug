
error[E0282]: type annotations needed for `[closure@src/main.rs:3:13: 6:6]`
 --> src/main.rs:4:9
  |
3 |     let x = || {
  |         - consider giving `x` the explicit type `[closure@src/main.rs:3:13: 6:6]`, with the type parameters specified
4 |         Err(())?;
  |         ^^^^^^^^ cannot infer type
