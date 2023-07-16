plain
---- src/thread/mod.rs - thread::available_parallelism (line 1487) stdout ----
error[E0658]: use of unstable library feature 'available_parallelism'
 --> src/thread/mod.rs:1493:17
  |
8 |     let count = thread::available_parallelism()?.get();
  |
  = note: see issue #74479 <https://github.com/rust-lang/rust/issues/74479> for more information
  = note: see issue #74479 <https://github.com/rust-lang/rust/issues/74479> for more information
  = help: add `#![feature(available_parallelism)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:21:15
