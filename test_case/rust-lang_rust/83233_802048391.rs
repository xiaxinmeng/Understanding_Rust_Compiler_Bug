plain
.................................................................................................... 9300/11700
.................................................................................................... 9400/11700
.................................................................................................... 9500/11700
...........................................i......i................................................. 9600/11700
.........................................................................................iiiiiii..ii 9700/11700
.................................................................................................... 9900/11700
.................................................................................................... 10000/11700
.................................................................................................... 10100/11700
.................................................................................................... 10200/11700
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.03s

 finished in 0.088 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.376 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

   Doc-tests core

running 2847 tests
iiiiii.................F............................................................................ 100/2847
....ii.............................................................................................. 300/2847
.....................................................i.............................................. 400/2847
.................................................................................................... 500/2847
..........................i..i...................iiii............................................... 600/2847
---
.....................i.....................i.....................i.................................. 2800/2847
...............................................
failures:

---- src/array/mod.rs - array::[T; N]::split_array_mut (line 593) stdout ----
error[E0658]: use of unstable library feature 'slice_split_array': new API
  |
  |
7 | let (left, right) = v.split_array_mut::<2>();
  |
  = note: see issue #74674 <https://github.com/rust-lang/rust/issues/74674> for more information
  = note: see issue #74674 <https://github.com/rust-lang/rust/issues/74674> for more information
  = help: add `#![feature(slice_split_array)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:12
