
error: implementation of `Foo` is not general enough
 --> src/main.rs:2:5
  |
2 |     test::<FooS>(&mut 42);
  |     ^^^^^^^^^^^^
  |
  = note: Due to a where-clause on `test`,
  = note: `FooS<'_>` must implement `Foo<'0>`, for any lifetime `'0`
  = note: but `FooS<'_>` actually implements `Foo<'1>`, for some specific lifetime `'1`
