
error[E0121]: the type placeholder `_` is not allowed within types on item signatures
 --> main.rs:1:24
  |
1 | pub const FOO: fn() -> _ = 1;
  |                        ^
  |                        |
  |                        not allowed in type signatures
  |                        help: use type parameters instead: `T`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0121`.

