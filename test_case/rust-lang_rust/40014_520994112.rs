
error[E0282]: type annotations needed for the closure
 --> file7.rs:2:17
  |
2 |     let _v = || [];
  |         --      ^^ cannot infer type
  |         |
  |         consider giving `_v` a boxed closure type like `Box<dyn Fn() -> [_; 0]>`
