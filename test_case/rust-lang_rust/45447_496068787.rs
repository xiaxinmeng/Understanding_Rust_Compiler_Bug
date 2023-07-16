
error[E0401]: can't use generic parameters from outer function
 --> src/lib.rs:3:14
  |
1 | fn foo<T>() {
  |        - type variable from outer function
2 |     //fn bar
  |          --- try adding a local generic parameter in this method instead
3 |     type U = T;
  |              ^ use of generic parameter from outer function
