
error: implementation of `Foo` is not general enough
  --> file.rs:2:5
   |
2  |     test::<FooS>(&mut 42);
   |     ^^^^^^^^^^^^
...
13 | fn test<'a, F>(data: &'a mut u32) where F: for<'b> Foo<'b> {}
   | ------------------------------------------------------------- due to a where-clause on `test`...
   |
   = note: ...`FooS<'_>` must implement `Foo<'0>`, for any lifetime `'0`...
   = note: ...but `FooS<'_>` actually implements `Foo<'1>`, for some specific lifetime `'1`
