
error[E0283]: type annotations needed
  --> src/main.rs:20:15
   |
20 |     let x = y.into() * 512u32;
   |             --^^^^--
   |             | |
   |             | cannot infer type for struct `Y`
   |             this method call resolves to `T`
   |
   = note: cannot satisfy `Y: std::convert::Into<_>`
