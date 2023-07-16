
error[E0106]: missing lifetime specifier
 --> src/lib.rs:5:17
  |
5 | fn context() -> Context {
  |                 ^^^^^^^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn context() -> Context<'static> {
  |                 ^^^^^^^^^^^^^^^^
