
error[E0046]: not all trait items implemented, missing: `N`
 --> src/main.rs:5:1
  |
2 |     const N: usize;
  |     --------------- `N` from trait
...
5 | impl Trait for i32 {}
  | ^^^^^^^^^^^^^^^^^^ missing `N` in implementation

For more information about this error, try `rustc --explain E0046`.
