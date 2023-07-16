plain
error[E0284]: type annotations needed
  --> src/lib.rs:17:9
   |
17 |         try_from_fn(|| -> Result<i32, TryFromIntError> { i32::try_from(i) })?,
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot satisfy `<_ as Try>::Residual == _`
