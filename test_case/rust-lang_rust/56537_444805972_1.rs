
error[E0106]: missing lifetime specifier
 --> issue-22340-a.rs:6:21
  |
6 |         .map(|s| -> &str { &s[..] }) // works
  |                     ^ help: consider giving it an explicit bounded or 'static lifetime: `&'static`
  |
  = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
