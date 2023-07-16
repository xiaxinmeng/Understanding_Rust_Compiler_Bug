
error[E0308]: mismatched types
  --> src/main.rs:10:5
   |
9  | fn main() {
   |           - expected `()` because of default return type
10 |     krate::T
   |     ^^^^^^^^ expected `()`, found struct `S`
