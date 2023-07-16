
error[E0401]: can't use generic parameters from outer function
 --> src/lib.rs:2:15
  |
1 | fn x<T: Default>() {
  |    - - type parameter from outer function
  |    |
  |    try adding a local generic parameter in this method instead
2 |     static a: T = T::default();
  |               ^ use of generic parameter from outer function

error[E0401]: can't use generic parameters from outer function
 --> src/lib.rs:2:19
  |
1 | fn x<T: Default>() {
  |    - - type parameter from outer function
  |    |
  |    try adding a local generic parameter in this method instead
2 |     static a: T = T::default();
  |                   ^^^^^^^^^^ use of generic parameter from outer function
