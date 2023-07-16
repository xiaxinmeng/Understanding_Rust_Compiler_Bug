
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:31
  |
1 | fn f(_: impl Iterator<Item = &'_ ()>) {}
  |                               ^^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 | fn f(_: 'a, impl Iterator<Item = &'a ()>) {}
  |         ^^                       ^^^
