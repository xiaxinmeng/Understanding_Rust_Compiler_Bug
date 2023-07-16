plain
.................................................................................................... 9300/11534
.................................................................................................... 9400/11534
..........................................................................i......i.................. 9500/11534
.................................................................................................... 9600/11534
.............iiiiiii..iiiiii.i...................................................................... 9700/11534
.................................................................................................... 9900/11534
.................................................................................................... 10000/11534
.................................................................................................... 10100/11534
.................................................................................................... 10200/11534
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.00s

 finished in 0.067 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.402 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................i.....................i.....................i...................................... 2800/2843
...........................................
failures:

---- src/iter/adapters/zip.rs - iter::adapters::zip::zip (line 42) stdout ----
error[E0433]: failed to resolve: use of undeclared crate or module `iter`
 --> src/iter/adapters/zip.rs:43:5
4 | use iter::iter::zip;
  |     ^^^^ use of undeclared crate or module `iter`


error[E0425]: cannot find function `zip` in this scope
 --> src/iter/adapters/zip.rs:47:15
  |
8 | for (x, y) in zip(&xs, &ys) {
  |
help: consider importing one of these items
  |
3 | use core::iter::zip;
3 | use core::iter::zip;
  |
3 | use std::iter::zip;
  |

error[E0425]: cannot find function `zip` in this scope
  --> src/iter/adapters/zip.rs:53:20
   |
14 | for ((x, y), z) in zip(zip(&xs, &ys), &zs) {
   |
help: consider importing one of these items
   |
3  | use core::iter::zip;
3  | use core::iter::zip;
   |
3  | use std::iter::zip;
   |

error[E0425]: cannot find function `zip` in this scope
  --> src/iter/adapters/zip.rs:53:24
   |
14 | for ((x, y), z) in zip(zip(&xs, &ys), &zs) {
   |
help: consider importing one of these items
   |
3  | use core::iter::zip;
---
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.

failures:
    src/iter/adapters/zip.rs - iter::adapters::zip::zip (line 42)
test result: FAILED. 2815 passed; 1 failed; 27 ignored; 0 measured; 0 filtered out; finished in 44.68s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:51
