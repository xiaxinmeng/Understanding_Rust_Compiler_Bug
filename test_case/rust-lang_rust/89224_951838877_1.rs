rust
error[E0425]: cannot find function, tuple struct or tuple variant `Wrapping` in this scope
 --> ...\test.rs:2:13
  |
2 |     let _ = Wrapping(5_u32);
  |             ^^^^^^^^ not found in this scope
  |
help: consider importing one of these items
  |
1 | use core::num::Wrapping;
  |
1 | use std::num::Wrapping;
  |
