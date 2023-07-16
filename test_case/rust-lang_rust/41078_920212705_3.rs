
error: implementation of `FnOnce` is not general enough
 --> src/main.rs:3:29
  |
3 |     vec![1,2,3].into_iter().find(suitable);
  |                             ^^^^ implementation of `FnOnce` is not general enough
  |
  = note: closure with signature `fn(&'2 i32) -> bool` must implement `FnOnce<(&'1 i32,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `FnOnce<(&'2 i32,)>`, for some specific lifetime `'2`
