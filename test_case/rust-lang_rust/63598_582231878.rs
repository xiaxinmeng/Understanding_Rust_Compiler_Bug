
error[E0106]: missing lifetime specifier
 --> src/lib.rs:5:17
  |
5 | async fn foo<T: Foo>() {
  |                 ^^^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
5 | async fn foo<'lifetime, T: Foo<'lifetime>>() {
  |              ^^^^^^^^^^    ^^^^^^^^^^^^^^
