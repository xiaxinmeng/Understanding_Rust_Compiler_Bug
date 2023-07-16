
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
1 | fn foo(bar: u64) -> u64 {
  |                     --- expected `u64` because of return type
2 |     6 as i64
  |     ^^^^^^^^ expected u64, found i64
