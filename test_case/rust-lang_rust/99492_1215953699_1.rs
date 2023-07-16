
error[E0308]: mismatched types
  --> src/lib.rs:40:5
   |
40 |     spawn(execution_process());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected reference `&u64`
              found reference `&u64`
note: the lifetime requirement is introduced here
  --> src/lib.rs:45:8
   |
45 |     T: Send, // zzz
   |        ^^^^

error: implementation of `FnOnce` is not general enough
  --> src/lib.rs:40:5
   |
40 |     spawn(execution_process());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'0 u64) -> u64` must implement `FnOnce<(&'1 u64,)>`, for any two lifetimes `'0` and `'1`...
   = note: ...but it actually implements `FnOnce<(&u64,)>`

For more information about this error, try `rustc --explain E0308`.
