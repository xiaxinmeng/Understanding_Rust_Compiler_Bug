
error[E0271]: type mismatch resolving `<() as Tr>::A == u8`
 --> src/lib.rs:9:11
  |
9 | fn f() -> impl Tr<A=u8> {
  |           ^^^^^^^^^^^^^ expected (), found u8
  |
  = note: expected type `()`
             found type `u8`
  = note: the return type of a function must have a statically known size
