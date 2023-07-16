rust
error[E0282]: type annotations needed for `Vec<T>`
 --> tests/ui/type/type-check/cannot_infer_local_or_vec.rs:2:9
  |
2 |     let x = vec![];
  |         ^
-Ztrack-diagnostics: created at compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:559:14
  |
help: consider giving `x` an explicit type, where the placeholders `_` are specified
  |
2 |     let x: Vec<T> = vec![];
  |          ++++++++
