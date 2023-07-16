
error: implementation of `Test` is not general enough
  --> src/main.rs:38:5
   |
38 |     do_test(test2);
   |     ^^^^^^^ implementation of `Test` is not general enough
   |
   = note: `fn(&mut i32) -> impl Future {test2::<'_, &mut i32>}` must implement `Test<'0>`, for any lifetime `'0`...
   = note: ...but it actually implements `Test<'1>`, for some specific lifetime `'1`
