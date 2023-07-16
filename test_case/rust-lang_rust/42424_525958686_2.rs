
error[E0284]: type annotations required: cannot resolve `<_ as std::ops::Try>::Ok == _`
  --> src/main.rs:11:20
   |
11 |         let x: X = new_x()?;
   |                    ^^^^^^^^
help: give a type to the `try` block without `_` type argument placeholders:
   |
10 |     let _: Result<_, _> = try {
   |     ^^^^^^^^^^^^^^^^^^^^^
