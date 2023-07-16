
error[E0282]: type annotations needed
  --> lib/ann/src/ann/cost_function.rs:25:9
   |
25 |         diff2.iter().sum() as f64 / 2.0
   |                      ^^^ cannot infer type of type parameter `S`
   |
   = note: type must be known at this point
