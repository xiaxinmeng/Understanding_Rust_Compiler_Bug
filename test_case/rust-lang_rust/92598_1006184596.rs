plain
..................................iiiiii.................................i.......................... 1200/1205
.....
failures:

---- src/panicking.rs - panicking::update_hook (line 196) stdout ----
error[E0658]: use of unstable library feature 'panic_update_hook'
  |
  |
6 | panic::update_hook(|prev| {
  |
  |
  = help: add `#![feature(panic_update_hook)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:25:13
