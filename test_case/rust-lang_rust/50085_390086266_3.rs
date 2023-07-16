none
error[E0308]: mismatched types
 --> b.rs:2:17
  |
1 | fn foo() {
  |          - help: try adding a return type: `-> bool`
2 |     let a = { { true } true };
  |                 ^^^^ expected (), found bool
  |
  = note: expected type `()`
             found type `bool`
