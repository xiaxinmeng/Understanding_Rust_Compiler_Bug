
error[E0283]: type annotations needed for the closure `fn() -> std::result::Result<(), _>`
  --> src/lib.rs:14:20
   |
14 |         may_error()?;
   |                    ^ cannot infer type
   |
   = note: cannot satisfy `_: From<()>`
   = note: required by `from`
help: give this closure an explicit return type without `_` placeholders
   |
13 |     let v = || -> std::result::Result<(), _> {
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
