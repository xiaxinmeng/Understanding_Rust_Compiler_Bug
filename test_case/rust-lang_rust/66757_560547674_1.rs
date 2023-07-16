rust
   Compiling playground v0.0.1 (/playground)
error[E0282]: type annotations needed
 --> src/lib.rs:6:5
  |
3 |     let x: _ = never;
  |         - consider giving `x` a type
...
6 |     x.bar;
  |     ^ cannot infer type
  |
  = note: type must be known at this point
