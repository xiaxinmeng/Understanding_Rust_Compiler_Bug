
error: implementation of `FnOnce` is not general enough
  --> src/main.rs:22:9
   |
22 |         tokio::task::spawn(async move {
   |         ^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'0 u64) -> u64` must implement `FnOnce<(&u64,)>`, for any lifetime `'0`...
   = note: ...but it actually implements `FnOnce<(&u64,)>`
