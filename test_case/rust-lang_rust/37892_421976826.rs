rust
error[E0401]: can't use type parameters from outer function
 --> src/main.rs:6:19
  |
4 | impl Foo {
  | ---- `Self` type implicitly declared here, by this `impl`
5 |     fn method<T>(&self) {
6 |         type X = (Self, T);
  |                   ^^^^
  |                   |
  |                   use of type variable from outer function
  |                   use a type here instead

error[E0401]: can't use type parameters from outer function
 --> src/main.rs:6:25
  |
5 |     fn method<T>(&self) {
  |        ------ - type variable from outer function
  |        |
  |        try adding a local type parameter in this method instead
6 |         type X = (Self, T);
  |                         ^ use of type variable from outer function

error: aborting due to 2 previous errors
