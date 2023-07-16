
error[E0044]: foreign items may not have type parameters
 --> src/main.rs:2:5
  |
2 |     fn sqrt<T>(f: T) -> T;
  |     ^^^^^^^^^^^^^^^^^^^^^^ can't have type parameters
  |
  = help: use specialization instead of type parameters by replacing them with concrete types like `u32`
