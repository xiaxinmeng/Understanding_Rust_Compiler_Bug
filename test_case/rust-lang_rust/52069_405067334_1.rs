
warning: implicit lifetime parameters in types are deprecated
  --> elide.rs:17:63
   |
17 | fn returns_double_lifetime(one: &'a str, another: &'b str) -> DoubleLifetime {
   |                                                               ^^^^^^^^^^^^^^
   |

[...]

error[E0106]: missing lifetime specifiers
  --> elide.rs:17:63
   |
17 | fn returns_double_lifetime(one: &'a str, another: &'b str) -> DoubleLifetime {
   |                                                               ^^^^^^^^^^^^^^ expected 2 lifetime parameters
