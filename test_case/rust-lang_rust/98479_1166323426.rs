plain
---- src/sync/atomic.rs - sync::atomic::AtomicBool::fetch_not (line 874) stdout ----
error[E0658]: use of unstable library feature 'atomic_bool_fetch_not'
 --> src/sync/atomic.rs:878:16
  |
7 | assert_eq!(foo.fetch_not(Ordering::SeqCst), true);
  |
  = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
  = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
  = help: add `#![feature(atomic_bool_fetch_not)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'atomic_bool_fetch_not'
  --> src/sync/atomic.rs:882:16
   |
   |
11 | assert_eq!(foo.fetch_not(Ordering::SeqCst), false);
   |
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = help: add `#![feature(atomic_bool_fetch_not)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
