
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:13
  |
1 | fn foo() -> &i32 {
  |             ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
1 | fn foo() -> &'static i32 {
  |             ^^^^^^^^

error: aborting due to previous error
