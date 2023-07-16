plain
---- src/mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit (line 34) stdout ----
error: use of mem::uninitialized is very likely to be undefined behavior
 --> src/mem/maybe_uninit.rs:38:24
  |
7 | let b: bool = unsafe { mem::uninitialized() }; // undefined behavior! ⚠️
  |
note: the lint level is defined here
 --> src/mem/maybe_uninit.rs:32:9
  |
---
---- src/mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit (line 49) stdout ----
error: use of mem::uninitialized is very likely to be undefined behavior
 --> src/mem/maybe_uninit.rs:53:23
  |
7 | let x: i32 = unsafe { mem::uninitialized() }; // undefined behavior! ⚠️
  |
note: the lint level is defined here
 --> src/mem/maybe_uninit.rs:47:9
  |
