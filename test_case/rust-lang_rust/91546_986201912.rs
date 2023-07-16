plain
.................................................................................................... 2800/3685
.................................................................................................... 2900/3685
.....iii............................................................................................ 3000/3685
.................................................................................................... 3100/3685
.....F.......F...................................................................................... 3200/3685
.......F.FF......................................................................................... 3300/3685
.....i.....................i.....................i.....................i............................ 3500/3685
....i.....................i.....................i.....................i.....................i....... 3600/3685
.....................................................................................
failures:
failures:

---- src/slice/iter.rs - slice::iter::SplitLeftInclusive (line 584) stdout ----
error[E0658]: use of unstable library feature 'split_left_inclusive'
  |
  |
5 | let mut iter = slice.split_left_inclusive(|num| num % 3 == 0);
  |
  |
  = help: add `#![feature(split_left_inclusive)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/slice/iter.rs - slice::iter::SplitLeftInclusiveMut (line 957) stdout ----
error[E0658]: use of unstable library feature 'split_left_inclusive'
  |
  |
5 | let iter = v.split_left_inclusive_mut(|num| *num % 3 == 0);
  |
  |
  = help: add `#![feature(split_left_inclusive)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::split_left_inclusive (line 1848) stdout ----
error[E0658]: use of unstable library feature 'split_left_inclusive'
  |
  |
5 | let mut iter = slice.split_left_inclusive(|num| num % 3 == 0);
  |
  |
  = help: add `#![feature(split_left_inclusive)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::split_left_inclusive (line 1861) stdout ----
error[E0658]: use of unstable library feature 'split_left_inclusive'
  |
  |
5 | let mut iter = slice.split_left_inclusive(|num| num % 3 == 0);
  |
  |
  = help: add `#![feature(split_left_inclusive)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::split_left_inclusive_mut (line 1884) stdout ----
error[E0658]: use of unstable library feature 'split_left_inclusive'
  |
  |
6 | for group in v.split_left_inclusive_mut(|num| *num % 3 == 0) {
  |
  |
  = help: add `#![feature(split_left_inclusive)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:22:57
