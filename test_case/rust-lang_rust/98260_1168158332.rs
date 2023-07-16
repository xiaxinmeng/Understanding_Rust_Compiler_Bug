
error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
 --> main.rs:3:27
  |
3 |     fn a(aa: B) -> Result<_, B> {
  |                    -------^----
  |                    |      |
  |                    |      not allowed in type signatures
  |                    help: replace with the correct return type: `Result<(), B>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0121`.
