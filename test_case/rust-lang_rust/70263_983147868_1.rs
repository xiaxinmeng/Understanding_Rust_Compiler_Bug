
error: implementation of `Foo` is not general enough
  --> src/main.rs:30:5
   |
30 |     run_with_ctx(move |arg: &mut A| async move { arg.test().await; });
   |     ^^^^^^^^^^^^ implementation of `Foo` is not general enough
   |
   = note: `[closure@src/main.rs:30:18: 30:64]` must implement `Foo<'1, A, ()>`, for any lifetime `'1`...
   = note: ...but it actually implements `Foo<'_, A, ()>`, for some specific lifetime `'2`
