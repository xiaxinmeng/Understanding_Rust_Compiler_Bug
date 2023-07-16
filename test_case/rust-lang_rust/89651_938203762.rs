plain
---- src/task/ready.rs - task::ready::ready (line 15) stdout ----
error[E0658]: use of unstable library feature 'ready_macro'
  --> src/task/ready.rs:24:15
   |
12 |     let num = ready!(fut.poll(cx));
   |
   = note: see issue #70922 <https://github.com/rust-lang/rust/issues/70922> for more information
   = note: see issue #70922 <https://github.com/rust-lang/rust/issues/70922> for more information
   = help: add `#![feature(ready_macro)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ready_macro'
 --> src/task/ready.rs:16:17
  |
  |
4 | use std::task::{ready, Context, Poll};
  |
  = note: see issue #70922 <https://github.com/rust-lang/rust/issues/70922> for more information
  = note: see issue #70922 <https://github.com/rust-lang/rust/issues/70922> for more information
  = help: add `#![feature(ready_macro)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:12
