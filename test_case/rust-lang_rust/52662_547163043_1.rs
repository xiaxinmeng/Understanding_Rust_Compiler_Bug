
error[E0391]: cycle detected when computing the supertraits of `B`
 --> src/main.rs:6:15
  |
6 | trait B: A<T: B> {}
  |               ^
  |
  = note: ...which again requires computing the supertraits of `B`, completing the cycle
note: cycle used when collecting item types in top-level module
 --> src/main.rs:6:1
  |
6 | trait B: A<T: B> {}
  | ^^^^^^^^^^^^^^^^
