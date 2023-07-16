
error[E0046]: not all trait items implemented, missing: `Output`
 --> src/main.rs:8:1
  |
8 | impl FnOnce<(isize, isize)> for Foo {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `Output` in implementation
  |
  = note: `Output` from trait: `type Output;`
