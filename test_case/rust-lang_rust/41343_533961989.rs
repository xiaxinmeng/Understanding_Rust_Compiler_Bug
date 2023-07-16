
error[E0106]: missing lifetime specifier
  --> src/lib.rs:15:33
   |
15 |     fn index(self, x: usize) -> &T {
   |                                 ^ help: consider giving it an explicit bounded or 'static lifetime: `&'static`
   |
   = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
