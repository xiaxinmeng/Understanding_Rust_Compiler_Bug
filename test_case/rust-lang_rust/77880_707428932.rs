
error[E0282]: type annotations needed for the closure `fn() -> std::result::Result<(), _>`
 --> src/lib.rs:5:9
  |
5 |         may_error()?;
  |         ^^^^^^^^^^^^ cannot infer type
  |
help: give this closure an explicit return type without `_` placeholders
  |
4 |     let g = || -> std::result::Result<(), _> {
  |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
