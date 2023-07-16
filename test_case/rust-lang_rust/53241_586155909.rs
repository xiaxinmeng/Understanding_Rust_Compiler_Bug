
error[E0401]: can't use generic parameters from outer function
 --> src/main.rs:8:22
  |
7 | fn baz<F: Foo>(a: F) {
  |        - type parameter from outer function
8 |     const QUX: i32 = F::BAR;
  |                      ^^^^^^ use of generic parameter from outer function
