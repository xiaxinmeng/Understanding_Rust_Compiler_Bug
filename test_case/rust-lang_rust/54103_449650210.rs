text
error[E0061]: this function takes 0 parameters but 1 parameter was supplied
 --> src/main.rs:6:7
  |
6 |     s.borrow(())
  |       ^^^^^^ expected 0 parameters

error[E0308]: mismatched types
 --> src/main.rs:6:5
  |
1 | fn main() {
  |           - expected `()` because of default return type
...
6 |     s.borrow(())
  |     ^^^^^^^^^^^^- help: try adding a semicolon: `;`
  |     |
  |     expected (), found &main::S
  |
  = note: expected type `()`
             found type `&main::S`
