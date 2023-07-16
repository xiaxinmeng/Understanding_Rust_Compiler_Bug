rust
error[E0121]: the type placeholder `_` is not allowed within types on item signatures
 --> <anon>:2:11
  |
2 | fn g() -> _ { f }
  |           ^
  |           |
  |           not allowed in type signatures
  |           help: replace with the correct return type: `fn() -> i32`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0121`.
