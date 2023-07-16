plain
---- src/thread/mod.rs - thread::Thread::into_raw (line 1240) stdout ----
error[E0658]: use of unstable library feature 'thread_raw'
 --> src/thread/mod.rs:1245:11
  |
8 | let ptr = Thread::into_raw(thread);
  |
  = note: see issue #97523 <https://github.com/rust-lang/rust/issues/97523> for more information
  = help: add `#![feature(thread_raw)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'thread_raw'
  --> src/thread/mod.rs:1247:16
   |
10 |     assert_eq!(Thread::from_raw(ptr).id(), id);
   |
   = note: see issue #97523 <https://github.com/rust-lang/rust/issues/97523> for more information
   = help: add `#![feature(thread_raw)]` to the crate attributes to enable

