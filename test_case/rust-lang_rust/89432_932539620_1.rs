text
error[E0277]: the trait bound `String: From<&str>` is not satisfied
 --> src/lib.rs:1:38
  |
1 | const FOO: fn() -> String = || "foo".into();
  |                                      ^^^^ the trait `From<&str>` is not implemented for `String`
  |
  = note: required because of the requirements on the impl of `Into<String>` for `&str`

error[E0277]: the trait bound `String: From<&str>` is not satisfied
 --> src/lib.rs:4:14
  |
4 |     || "bar".into()
  |              ^^^^ the trait `From<&str>` is not implemented for `String`
  |
  = note: required because of the requirements on the impl of `Into<String>` for `&str`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
  |
3 | pub fn bar() -> fn() -> String where String: From<&str> {
  |                                ++++++++++++++++++++++++
