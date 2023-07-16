
error[E0700]: hidden type for `impl Iterator<Item = String> + 'static` captures lifetime that does not appear in bounds
 --> src/lib.rs:9:5
  |
8 | fn bar(a: &str) -> impl Iterator<Item = String> + 'static {
  |           ---- hidden type `impl Iterator<Item = String> + 'static` captures the anonymous lifetime defined here
9 |     foo(a)
  |     ^^^^^^
