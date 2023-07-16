
error[E0401]: can't use type parameters from outer function; try using a local type parameter instead
 --> src/main.rs:8:25
5 | impl<T> Iterator for A<T> {
  | ^^^^ `Self` type implicitly declared here, on the `impl`
...
  |
8 |         fn helper(sel: &Self) -> u8 {
  |                         ^^^^ use of type variable from outer function
