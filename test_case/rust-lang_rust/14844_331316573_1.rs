
error[E0401]: can't use type parameters from outer function
  --> src/main.rs:5:20
3 | pub fn foo<T>() {
  |            - type variable from outer function
...
5 |     impl Bar for S<T> {
  |                    ^ use of type variable from outer function
note: try using a local type parameter instead
  |
5 |     impl<T> Bar for S<T> {
  |         ^^^
