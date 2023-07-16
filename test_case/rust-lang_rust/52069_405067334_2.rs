
error[E0106]: missing lifetime specifiers
  --> elide.rs:17:78
   |
17 | fn returns_double_lifetime(one: &'a str, another: &'b str) -> DoubleLifetime<'_, '_> {
   |                                                                              ^^ expected 2 lifetime parameters
