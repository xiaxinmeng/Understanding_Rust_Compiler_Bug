none
error[E0308]: mismatched types
 --> a.rs:2:17
  |
1 | fn main() {
  |           - expected `()` because of default return type
2 |     let a = { { true } true };
  |                 ^^^^ expected (), found bool
  |
  = note: expected type `()`
             found type `bool`
