rust
error: implementation of `Callable` is not general enough
  --> src/main.rs:28:5
   |
28 |     take_callable(|v: Value| Value::new());
   |     ^^^^^^^^^^^^^ implementation of `Callable` is not general enough
   |
   = note: `[closure@src/main.rs:28:19: 28:42]` must implement `Callable<'0>`, for any lifetime `'0`...
   = note: ...but it actually implements `Callable<'1>`, for some specific lifetime `'1`
