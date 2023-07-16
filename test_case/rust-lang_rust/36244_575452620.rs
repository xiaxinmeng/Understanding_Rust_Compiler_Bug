
error[E0284]: type annotations needed
 --> src/main.rs:9:8
  |
9 |     }).collect()?;
  |        ^^^^^^^
  |        |
  |        cannot infer type
  |        help: consider specifying the type argument in the method call: `collect::<B>`
  |
  = note: cannot resolve `<_ as std::ops::Try>::Ok == _`
