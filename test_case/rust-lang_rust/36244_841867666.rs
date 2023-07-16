
error[E0283]: type annotations needed
 --> src/main.rs:9:8
  |
9 |     }).collect()?;
  |        ^^^^^^^ cannot infer type
  |
  = note: cannot satisfy `_: Try`
  = note: required by `into_result`
help: consider specifying the type argument in the method call
  |
9 |     }).collect::<B>()?;
  |               ^^^^^
