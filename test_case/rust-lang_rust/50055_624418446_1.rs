
error[E0637]: `'_` cannot be used here
 --> src/lib.rs:1:10
  |
1 | struct S<'_> {
  |          ^^ `'_` is a reserved lifetime name

error[E0106]: missing lifetime specifier
 --> src/lib.rs:2:9
  |
2 |     x: &'_ (),
  |         ^^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 | struct S<'a, '_> {
2 |     x: &'a (),
  |
