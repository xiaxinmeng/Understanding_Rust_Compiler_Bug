plain
....................i.....................i.....................i.....................i............. 2800/2868
....................................................................
failures:

---- src/convert/mod.rs - convert::Infallible (line 662) stdout ----
error[E0119]: conflicting implementations of trait `main::_doctest_main_src_convert_mod_rs_662_0::MyTrait` for type `fn() -> !`
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


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:58
