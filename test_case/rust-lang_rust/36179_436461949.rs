
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:15
  |
1 | fn hello() -> &str {
  |               ^ help: consider giving it a 'static lifetime: `&'static`
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
