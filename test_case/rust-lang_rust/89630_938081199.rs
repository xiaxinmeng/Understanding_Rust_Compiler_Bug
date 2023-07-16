plain

running 3599 tests
iiiiiiii............................................................................................ 100/3599
.................................................................................................... 200/3599
.........ii..F...................................................................................... 300/3599
.................................................................................................... 500/3599
.........................................i.i...............iiii..................................... 600/3599
.................................................................................................... 700/3599
.................................................................................................... 800/3599
---
...................i.....................i.....................i.....................i.............. 3500/3599
.......i...........................................................................................
failures:

---- src/convert/mod.rs - convert::Infallible (line 664) stdout ----
error[E0119]: conflicting implementations of trait `main::_doctest_main_src_convert_mod_rs_664_0::MyTrait` for type `fn() -> !`
  |
  |
5 | impl MyTrait for fn() -> ! {}
  | -------------------------- first implementation here
6 | impl MyTrait for fn() -> std::convert::Infallible {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `fn() -> !`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0119`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:22:29
