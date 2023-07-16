
   Compiling playground v0.0.1 (/playground)
error[E0282]: type annotations needed
 --> src/lib.rs:2:7
  |
2 |   A = foo(),
  |       ^^^ cannot infer type for `T`

error[E0019]: constant contains unimplemented expression type
 --> src/lib.rs:3:7
  |
3 |   B = foo(), // <- error: internal compiler error: src/librustc/ty/mod.rs:2400: enum discriminant depends on generic arguments
  |       ^^^^^

error: internal compiler error: src/librustc/ty/mod.rs:2394: enum discriminant depends on generic arguments
 --> src/lib.rs:3:7
  |
3 |   B = foo(), // <- error: internal compiler error: src/librustc/ty/mod.rs:2400: enum discriminant depends on generic arguments
  |    
