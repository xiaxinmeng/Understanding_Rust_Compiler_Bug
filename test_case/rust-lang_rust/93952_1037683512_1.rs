
error[E0391]: cycle detected when computing type of `main::<impl at src/main.rs:4:1: 8:2>::foo::<impl at src/main.rs:6:9: 6:34>`
 --> src/main.rs:6:14
  |
6 |         impl Self { fn bar() {} }
  |              ^^^^
  |
  = note: ...which immediately requires computing type of `main::<impl at src/main.rs:4:1: 8:2>::foo::<impl at src/main.rs:6:9: 6:34>` again
note: cycle used when collecting item types in top-level module
 --> src/main.rs:1:1
  |
1 | fn main() {
  | ^^^^^^^^^
  