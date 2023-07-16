
error[E0283]: type annotations needed for `Y`
  --> src/main.rs:20:15
   |
19 |     let y = Y::new();
   |         - consider giving `y` a type
20 |     let x = y.into() * 512u32;
   |               ^^^^ cannot infer type for struct `Y`
   |
   = note: cannot resolve `Y: std::convert::Into<_>`
