
error[E0284]: type annotations needed
 --> file12.rs:2:29
  |
2 |     vec![Ok(2)].into_iter().collect()?;
  |                             ^^^^^^^
  |                             |
  |                             cannot infer type
  |                             help: consider specifying the type argument in the method call: `collect::<B>`
  |
  = note: cannot resolve `<_ as std::ops::Try>::Ok == _`
