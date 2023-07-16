
error[E0308]: mismatched types
 --> src/main.rs:3:5
  |
2 | async fn main() {
  |                 - expected `()` because of default return type
3 |     Ok(())
  |     ^^^^^^ expected `()`, found `Result<(), _>`
  |
  = note: expected unit type `()`
                  found enum `Result<(), _>`
