plain
---- src/backtrace.rs - backtrace (line 72) stdout ----
error[E0658]: use of unstable library feature 'backtrace'
 --> src/backtrace.rs:73:1
  |
4 | std::backtrace::set_library(std::backtrace::CapturePreference::Full);
  |
  = note: see issue #53487 <https://github.com/rust-lang/rust/issues/53487> for more information
  = note: see issue #53487 <https://github.com/rust-lang/rust/issues/53487> for more information
  = help: add `#![feature(backtrace)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'backtrace'
 --> src/backtrace.rs:73:29
  |
  |
4 | std::backtrace::set_library(std::backtrace::CapturePreference::Full);
  |
  = note: see issue #53487 <https://github.com/rust-lang/rust/issues/53487> for more information
  = note: see issue #53487 <https://github.com/rust-lang/rust/issues/53487> for more information
  = help: add `#![feature(backtrace)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'backtrace'
 --> src/backtrace.rs:74:1
  |
  |
5 | std::backtrace::set_panic(std::backtrace::CapturePreference::Full);
  |
  = note: see issue #53487 <https://github.com/rust-lang/rust/issues/53487> for more information
  = note: see issue #53487 <https://github.com/rust-lang/rust/issues/53487> for more information
  = help: add `#![feature(backtrace)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'backtrace'
 --> src/backtrace.rs:74:27
  |
  |
5 | std::backtrace::set_panic(std::backtrace::CapturePreference::Full);
  |
  = note: see issue #53487 <https://github.com/rust-lang/rust/issues/53487> for more information
  = note: see issue #53487 <https://github.com/rust-lang/rust/issues/53487> for more information
  = help: add `#![feature(backtrace)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:23:43
