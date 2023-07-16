
error[E0401]: can't use generic parameters from outer function
 --> file.rs:4:17
  |
3 | fn bar<T: Foo>(n: T) {
  |    --- - type variable from outer function
  |    |
  |    try adding a local generic parameter in this method instead
4 |     const BASE: T = T::FOO;
  |                 ^ use of generic parameter from outer function

error[E0401]: can't use generic parameters from outer function
 --> file.rs:4:21
  |
3 | fn bar<T: Foo>(n: T) {
  |    --- - type variable from outer function
  |    |
  |    try adding a local generic parameter in this method instead
4 |     const BASE: T = T::FOO;
  |                     ^^^^^^ use of generic parameter from outer function
