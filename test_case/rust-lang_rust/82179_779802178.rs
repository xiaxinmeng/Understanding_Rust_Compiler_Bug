plain
.................................................................................................... 9200/11455
.................................................................................................... 9300/11455
.................................................................................................... 9400/11455
..............i......i.............................................................................. 9500/11455
....................................................iiiiiii..iiiiii.i............................... 9600/11455
.................................................................................................... 9800/11455
.................................................................................................... 9900/11455
.................................................................................................... 10000/11455
.................................................................................................... 10100/11455
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.076 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.541 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.41s

 finished in 2.494 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................i.................. 2500/2839
.............................i................i.....................i.....................i......... 2600/2839
............i.....................i................................i.....................i.......... 2700/2839
...........i.....................i.....................i............................................ 2800/2839
.........................F.F...........

---- src/time.rs - time::Duration::try_from_secs_f32 (line 774) stdout ----
---- src/time.rs - time::Duration::try_from_secs_f32 (line 774) stdout ----
error[E0658]: use of unstable library feature 'duration_checked_float'
  |
  |
6 | let dur = Duration::try_from_secs_f32(2.7);
  |
  |
  = help: add `#![feature(duration_checked_float)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'duration_checked_float'
  |
  |
9 | let negative = Duration::try_from_secs_f32(-5.0);
  |
  |
  = help: add `#![feature(duration_checked_float)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/time.rs - time::Duration::try_from_secs_f64 (line 707) stdout ----
error[E0658]: use of unstable library feature 'duration_checked_float'
  |
  |
6 | let dur = Duration::try_from_secs_f64(2.7);
  |
  |
  = help: add `#![feature(duration_checked_float)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'duration_checked_float'
  |
  |
9 | let negative = Duration::try_from_secs_f64(-5.0);
  |
  |
  = help: add `#![feature(duration_checked_float)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:22:05
