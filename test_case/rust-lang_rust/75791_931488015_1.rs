
error: implementation of `FnOnce` is not general enough
 --> src/main.rs:5:5
  |
5 |     thing(f);
  |     ^^^^^ implementation of `FnOnce` is not general enough
  |
  = note: closure with signature `fn(&'2 u32)` must implement `FnOnce<(&'1 u32,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `FnOnce<(&'2 u32,)>`, for some specific lifetime `'2`
