
error[E0283]: type annotations needed
  --> src/main.rs:13:15
   |
13 |     assert!(x != y.into());
   |               ^^ -------- this method call resolves to `T`
   |               |
   |               cannot infer type
   |
   = note: cannot satisfy `u32: PartialEq<_>`
