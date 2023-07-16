
error[E0308]: mismatched types
 --> src/main.rs:2:6
  |
1 | fn main() {
  |           - expected `()` because of default return type
2 |     {0}.. {}
  |      ^ expected (), found integral variable
  |
  = note: expected type `()`
             found type `{integer}`

error[E0308]: mismatched types
 --> src/main.rs:2:8
  |
1 | fn main() {
  |           - expected `()` because of default return type
2 |     {0}.. {}
  |        -----
  |        |
  |        expected (), found struct `std::ops::RangeTo`
  |        in this macro invocation
  |
  = note: expected type `()`
             found type `std::ops::RangeTo<()>`
