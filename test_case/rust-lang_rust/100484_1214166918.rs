
error[E0038]: the trait alias `SelfInput` cannot be made into an object
 --> src/main.rs:5:19
  |
5 | pub fn f(_f: &dyn SelfInput) {}
  |                   ^^^^^^^^^
  |
  = note: it cannot use `Self` as a type parameter in a supertrait or `where`-clause
