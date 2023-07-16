

error[E0106]: missing lifetime specifier
 --> src/main.rs:1:36
  |
1 | fn foo(x: &i32, y: &i32) -> Option<&i32> {
  |           ----     ----            ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
1 | fn foo<'a>(x: &'a i32, y: &'a i32) -> Option<&'a i32> {
  |       ++++     ++          ++                 ++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0106`.

