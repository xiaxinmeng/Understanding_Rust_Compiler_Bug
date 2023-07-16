
error[[E0282]](https://doc.rust-lang.org/stable/error-index.html#E0282): type annotations needed
 --> src/lib.rs:2:18
  |
2 |     diff2.iter().sum() as f64 / 2.0
  |                  ^^^ cannot infer type of the type parameter `S` declared on the associated function `sum`
  |
help: consider specifying the generic argument
  |
2 |     diff2.iter().sum::<S>() as f64 / 2.0
  |                     +++++

For more information about this error, try `rustc --explain E0282`.
