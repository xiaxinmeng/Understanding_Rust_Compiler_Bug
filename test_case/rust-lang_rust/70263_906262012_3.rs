
error: implementation of `FnOnceShim` is not general enough
 --> src/mutex.rs:118:13
  |
7 | let guard = MutexGuard::map(
  |             ^^^^^^^^^^^^^^^ implementation of `FnOnceShim` is not general enough
  |
  = note: `[closure@src/mutex.rs:9:5: 9:38]` must implement `FnOnceShim<'0, &'0 mut (i32, i32)>`, for any lifetime `'0`...
  = note: ...but it actually implements `FnOnceShim<'_, &'any mut (i32, i32)>`
