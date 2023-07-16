none
error[E0308]: mismatched types
 --> c.rs:2:17
  |
1 | fn foo() -> () {
  |             -- expected `()` because of return type
2 |     let a = { { true } true };
  |                 ^^^^ expected (), found bool
  |
  = note: expected type `()`
             found type `bool`
