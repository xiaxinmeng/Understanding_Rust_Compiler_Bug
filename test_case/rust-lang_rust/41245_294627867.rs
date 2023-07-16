
error[E0072]: recursive type `A` has infinite size
 --> file2.rs:1:1
  |
1 |   struct A {
  |  _^ starting here...
2 | |     a: A,
3 | \ }
  |
