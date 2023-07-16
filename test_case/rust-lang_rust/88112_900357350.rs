plain
.................................................................................................... 100/598
.................................................................................................... 200/598
................................................i................................................... 300/598
.................................................................................................... 400/598
................................................................F.............F..................... 500/598
failures:

---- src/sync.rs - sync::Arc<T>::lock_strong_count (line 791) stdout ----
error[E0658]: use of unstable library feature 'lock_arc'
error[E0658]: use of unstable library feature 'lock_arc'
 --> src/sync.rs:795:4
  |
7 | if Arc::lock_strong_count(&mut x) {
  |
  |
  = help: add `#![feature(lock_arc)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'get_mut_unchecked'
 --> src/sync.rs:797:15
  |
  |
9 |     unsafe { *Arc::get_mut_unchecked(&mut x) = "goodbye".to_owned(); }
  |
  = note: see issue #63292 <https://github.com/rust-lang/rust/issues/63292> for more information
  = note: see issue #63292 <https://github.com/rust-lang/rust/issues/63292> for more information
  = help: add `#![feature(get_mut_unchecked)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'lock_arc'
  --> src/sync.rs:798:5
   |
10 |     Arc::unlock_strong_count(&mut x);
10 |     Arc::unlock_strong_count(&mut x);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add `#![feature(lock_arc)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync.rs - sync::Arc<T>::unlock_strong_count (line 814) stdout ----
error[E0658]: use of unstable library feature 'lock_arc'
 --> src/sync.rs:818:4
  |
7 | if Arc::lock_strong_count(&mut x) {
  |
  |
  = help: add `#![feature(lock_arc)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'get_mut_unchecked'
 --> src/sync.rs:820:15
  |
  |
9 |     unsafe { *Arc::get_mut_unchecked(&mut x) = "goodbye".to_owned(); }
  |
  = note: see issue #63292 <https://github.com/rust-lang/rust/issues/63292> for more information
  = note: see issue #63292 <https://github.com/rust-lang/rust/issues/63292> for more information
  = help: add `#![feature(get_mut_unchecked)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'lock_arc'
  --> src/sync.rs:821:5
   |
10 |     Arc::unlock_strong_count(&mut x);
10 |     Arc::unlock_strong_count(&mut x);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add `#![feature(lock_arc)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:17:42
