plain
   Doc-tests core


running 3680 tests
.................................i..i......iiiiii.......................................F..F........ 100/3680
......................................................................ii............................ 300/3680
.................................................................................................... 400/3680
.....................i.............................................................................. 500/3680
.................................................................................................... 600/3680
---
.....................i.....................i.....................i.....................i............ 3600/3680
................................................................................
failures:

---- src/array/mod.rs - array::[T;N]::rsplit_array_ref (line 682) stdout ----
error[E0277]: can't compare `[{integer}; 0]` with `[{integer}; 6]`
   |
   |
10 |    assert_eq!(left, &[1, 2, 3, 4, 5, 6]);
   |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `[{integer}; 0] == [{integer}; 6]`
   |
   = help: the trait `PartialEq<[{integer}; 6]>` is not implemented for `[{integer}; 0]`
   = note: required because of the requirements on the impl of `PartialEq<&[{integer}; 6]>` for `&[{integer}; 0]`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: can't compare `[{integer}; 2]` with `[{integer}; 4]`
   |
   |
16 |     assert_eq!(left, &[1, 2, 3, 4]);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `[{integer}; 2] == [{integer}; 4]`
   |
   = help: the trait `PartialEq<[{integer}; 4]>` is not implemented for `[{integer}; 2]`
   = note: required because of the requirements on the impl of `PartialEq<&[{integer}; 4]>` for `&[{integer}; 2]`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: can't compare `[{integer}; 6]` with `[_; 0]`
   |
   |
22 |     assert_eq!(left, &[]);
   |     ^^^^^^^^^^^^^^^^^^^^^ no implementation for `[{integer}; 6] == [_; 0]`
   |
   = help: the trait `PartialEq<[_; 0]>` is not implemented for `[{integer}; 6]`
   = note: required because of the requirements on the impl of `PartialEq<&[_; 0]>` for `&[{integer}; 6]`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/array/mod.rs - array::[T;N]::rsplit_array_mut (line 727) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[1, 0, 3, 0]`,
 right: `[1, 0]`', src/array/mod.rs:8:1


---- src/slice/mod.rs - slice::[T]::rsplit_array_mut (line 1766) stdout ----
---- src/slice/mod.rs - slice::[T]::rsplit_array_mut (line 1766) stdout ----
error[E0277]: can't compare `[{integer}; 4]` with `[{integer}; 2]`
  |
  |
8 | assert_eq!(left, &mut [1, 0]);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `[{integer}; 4] == [{integer}; 2]`
  |
  = help: the trait `PartialEq<[{integer}; 2]>` is not implemented for `[{integer}; 4]`
  = note: required because of the requirements on the impl of `PartialEq<&mut [{integer}; 2]>` for `&mut [{integer}; 4]`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:21:39
