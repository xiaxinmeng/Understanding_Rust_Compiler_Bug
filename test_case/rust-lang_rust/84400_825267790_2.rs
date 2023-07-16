
error: implementation of `FnOnce` is not general enough
 --> src/main.rs:7:33
  |
7 |     let _ = options.into_iter().filter(check::<&String>);
  |                                 ^^^^^^ implementation of `FnOnce` is not general enough
  |
  = note: `fn(&'2 String) -> bool {check::<&'2 String>}` must implement `FnOnce<(&'1 String,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `FnOnce<(&'2 String,)>`, for some specific lifetime `'2`
