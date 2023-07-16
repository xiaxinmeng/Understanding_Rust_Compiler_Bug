
error[E0433]: failed to resolve: use of undeclared type `Cstring`
 --> src/main.rs:4:13
  |
4 |     let s = Cstring::new("hello").unwrap();
  |             ^^^^^^^
  |             |
  |             use of undeclared type `Cstring`
  |             help: a struct with a similar name exists (notice the capitalization): `CString`

For more information about this error, try `rustc --explain E0433`.
