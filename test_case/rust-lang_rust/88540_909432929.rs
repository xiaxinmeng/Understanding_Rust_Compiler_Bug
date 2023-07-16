plain
---- src/slice/mod.rs - slice::[T]::swap_unchecked (line 586) stdout ----
error[E0658]: use of unstable library feature 'slice_swap_unchecked'
 --> src/slice/mod.rs:589:12
  |
6 | unsafe { v.swap_unchecked(1, 3) };
  |
  = note: see issue #88539 <https://github.com/rust-lang/rust/issues/88539> for more information
  = note: see issue #88539 <https://github.com/rust-lang/rust/issues/88539> for more information
  = help: add `#![feature(slice_swap_unchecked)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:15:12
