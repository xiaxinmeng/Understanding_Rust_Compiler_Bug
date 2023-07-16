plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.072 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii...........iiii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.31s

 finished in 2.387 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1900/2831
.................................................................................................... 2000/2831
.................................................................................................... 2100/2831
.................................................................................................... 2200/2831
.............................................................FFF.F.................................. 2300/2831
...........................................................................i........................ 2500/2831
.......................i................i.....................i.....................i............... 2600/2831
......i.....................i................................i.....................i................ 2700/2831
.....i.....................i.....................i.................................................. 2800/2831
.....i.....................i.....................i.................................................. 2800/2831
...............................
failures:

---- src/slice/index.rs - slice::index::range (line 489) stdout ----
error[E0425]: cannot find function `range` in module `slice`
  |
  |
8 | slice::range(2..1, ..3);
  |        ^^^^^ not found in `slice`
help: consider importing this function
  |
5 | use core::slice::range;
  |
  |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/slice/index.rs - slice::index::range (line 497) stdout ----
error[E0425]: cannot find function `range` in module `slice`
  |
  |
8 | slice::range(1..4, ..3);
  |        ^^^^^ not found in `slice`
help: consider importing this function
  |
5 | use core::slice::range;
  |
  |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/slice/index.rs - slice::index::range (line 476) stdout ----
error[E0425]: cannot find function `range` in module `slice`
  |
  |
9 | assert_eq!(1..2, slice::range(1..2, ..v.len()));
  |                         ^^^^^ not found in `slice`
help: consider importing this function
  |
5 | use core::slice::range;
  |
  |

error[E0425]: cannot find function `range` in module `slice`
   |
   |
10 | assert_eq!(0..2, slice::range(..2, ..v.len()));
   |                         ^^^^^ not found in `slice`
help: consider importing this function
   |
5  | use core::slice::range;
   |
   |

error[E0425]: cannot find function `range` in module `slice`
   |
   |
11 | assert_eq!(1..3, slice::range(1.., ..v.len()));
   |                         ^^^^^ not found in `slice`
help: consider importing this function
   |
5  | use core::slice::range;
   |
   |

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/slice/index.rs - slice::index::range (line 505) stdout ----
error[E0425]: cannot find function `range` in module `slice`
  |
  |
8 | slice::range(1..=usize::MAX, ..3);
  |        ^^^^^ not found in `slice`
help: consider importing this function
  |
5 | use core::slice::range;
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:52
