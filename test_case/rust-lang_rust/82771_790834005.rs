plain
.................................................................................................... 9300/11526
.................................................................................................... 9400/11526
...........................................................................i......i................. 9500/11526
.................................................................................................... 9600/11526
..............iiiiiii..iiiiii.i..................................................................... 9700/11526
.................................................................................................... 9900/11526
.................................................................................................... 10000/11526
.................................................................................................... 10100/11526
.................................................................................................... 10200/11526
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.068 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.395 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
..............i.....................i.....................i......................................... 2800/2840
........................................
failures:

---- src/slice/iter.rs - slice::iter::IterMut::as_slice (line 288) stdout ----
error: the feature `slice_iter_mut_as_slice` has been stable since 1.52.0 and no longer requires an attribute to enable
  |
3 | #![feature(slice_iter_mut_as_slice)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:50
