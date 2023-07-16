
error[E0493]: destructors cannot be evaluated at compile-time
 --> src/lib.rs:4:25
  |
4 |     const fn const_drop(self) {
  |                         ^^^^ constant functions cannot evaluate destructors
5 |     }
  |     - value is dropped here
