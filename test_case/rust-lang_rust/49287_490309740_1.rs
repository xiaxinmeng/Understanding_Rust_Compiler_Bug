
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:31
  |
1 | fn f(_: impl Iterator<Item = &'_ ()>) {}
  |                               ^^ expected lifetime parameter
