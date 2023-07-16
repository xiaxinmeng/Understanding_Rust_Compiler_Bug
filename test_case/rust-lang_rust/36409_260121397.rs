
error[E0308]: mismatched types
 --> foo.rs:6:5
  |
5 | fn foo() {
  | -------- possibly return type `{integer}` missing here?
6 |     3
  |     ^
  |     |
  |     expected (), found integral variable
  |
  = note: expected type `()`
  = note:    found type `{integer}`
suggestion:
  |
6 |     3
  |      ^ possibly missing ';' here

