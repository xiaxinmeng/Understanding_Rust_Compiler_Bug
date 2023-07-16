
error[E0106]: missing lifetime specifier
 --> src/lib.rs:5:10
  |
5 |     x: X<'_>,
  |          ^^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
4 | struct S<'a> {
5 |     x: X<'a>,
  |
