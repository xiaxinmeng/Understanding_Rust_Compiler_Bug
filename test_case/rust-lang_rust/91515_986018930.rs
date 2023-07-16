plain
.....................i.....................i.....................i.....................i............ 3600/3680
................................................................................
failures:

---- src/slice/mod.rs - slice::[T]::rsplit_array_mut (line 1766) stdout ----
error[E0277]: can't compare `&mut [{integer}; 4]` with `[{integer}; 4]`
  |
  |
9 | assert_eq!(right, [3, 0, 5, 6]);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&mut [{integer}; 4] == [{integer}; 4]`
  |
  = help: the trait `PartialEq<[{integer}; 4]>` is not implemented for `&mut [{integer}; 4]`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:21:43
