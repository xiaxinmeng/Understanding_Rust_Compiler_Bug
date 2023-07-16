
error[E0061]: this function takes 2 parameters but 1 parameter was supplied
  --> src/test/compile-fail/E0061.rs:14:5
   |
11 | fn f(a: u16, b: &str) {}
   | - when calling this function
...
14 |     f(0);
   |     ^^^^ expected 2 parameters
   |
