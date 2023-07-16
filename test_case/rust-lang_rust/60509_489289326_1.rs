text
warning: variable does not need to be mutable
 --> code\src\lib.rs:8:22
  |
8 | async fn bar(n: u32, mut vec: Vec<u32>) {
  |                      ----^^^
  |                      |
  |                      help: remove this `mut`
  |
  = note: #[warn(unused_mut)] on by default
