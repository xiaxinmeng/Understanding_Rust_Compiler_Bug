
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:33
  |
1 | fn sync_fn(_: &i32, f: &u32) -> &u32 {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from argument 1 or `f`
help: consider introducing a named lifetime parameter
  |
1 | fn sync_fn<'a>(_: &'a i32, f: &'a u32) -> &'a u32 {
  |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^

For more information about this error, try `rustc --explain E0106`.
error: could not compile `playground` due to previous error
