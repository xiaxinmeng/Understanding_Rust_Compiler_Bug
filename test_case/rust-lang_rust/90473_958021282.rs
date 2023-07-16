
error: there is no argument named `x`
 --> src/main.rs:2:21
  |
2 |     tracing::info!("{x}");
  |                     ^^^
  |
  = help: if you intended to capture `x` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
