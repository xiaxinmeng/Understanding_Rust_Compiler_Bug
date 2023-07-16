plain
.................................................................................................... 9200/11472
.................................................................................................... 9300/11472
.................................................................................................... 9400/11472
.............................i......i............................................................... 9500/11472
....................................................................iiiiiii..iiiiii..i.............. 9600/11472
.................................................................................................... 9800/11472
.................................................................................................... 9900/11472
.................................................................................................... 10000/11472
.................................................................................................... 10100/11472
---
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
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.35s

 finished in 2.429 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1600/2842
.................................................................................................... 1700/2842
.................................................................................................... 1800/2842
.................................................................................................... 1900/2842
........................................................................F.................F......... 2000/2842
.................................................................................................... 2200/2842
.................................................................................................... 2300/2842
.................................................................................................... 2400/2842
......................................................................................i............. 2500/2842
......................................................................................i............. 2500/2842
..................................i................i.....................i.....................i.... 2600/2842
.................i.....................i................................i.....................i..... 2700/2842
................i.....................i.....................i....................................... 2800/2842
..........................................
failures:

---- src/ops/arith/wrapping.rs - ops::arith::wrapping::WrappingAdd::wrapping_add (line 18) stdout ----
error[E0658]: use of unstable library feature 'wrapping_add'
 --> src/ops/arith/wrapping.rs:20:12
  |
5 | assert_eq!(<u16 as WrappingAdd>::wrapping_add(100, 27), 127);
  |
  |
  = help: add `#![feature(wrapping_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'wrapping_add'
 --> src/ops/arith/wrapping.rs:21:12
  |
6 | assert_eq!(<u16 as WrappingAdd>::wrapping_add(u16::MAX, 2), 1);
  |
  |
  = help: add `#![feature(wrapping_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'wrapping_add'
 --> src/ops/arith/wrapping.rs:19:5
4 | use std::ops::WrappingAdd;
  |     ^^^^^^^^^^^^^^^^^^^^^
  |
  |
  = help: add `#![feature(wrapping_add)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ops/arith/wrapping.rs - ops::arith::wrapping::WrappingAddAssign::wrapping_add_assign (line 64) stdout ----
error[E0658]: use of unstable library feature 'wrapping_add'
 --> src/ops/arith/wrapping.rs:65:5
  |
4 | use std::ops::WrappingAddAssign;
  |
  |
  = help: add `#![feature(wrapping_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'wrapping_add'
 --> src/ops/arith/wrapping.rs:67:14
  |
6 | assert_eq!(x.wrapping_add_assign(1), 43);
  |
  |
  = help: add `#![feature(wrapping_add)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> src/ops/arith/wrapping.rs:67:1
  |
  |
6 | assert_eq!(x.wrapping_add_assign(1), 43);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found integer
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0658]: use of unstable library feature 'wrapping_add'
 --> src/ops/arith/wrapping.rs:68:14
  |
7 | assert_eq!(x.wrapping_add_assign(u16::MAX), 42);
  |
  |
  = help: add `#![feature(wrapping_add)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> src/ops/arith/wrapping.rs:68:1
  |
  |
7 | assert_eq!(x.wrapping_add_assign(u16::MAX), 42);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found integer
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 5 previous errors


Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
Couldn't compile the test.

failures:
    src/ops/arith/wrapping.rs - ops::arith::wrapping::WrappingAdd::wrapping_add (line 18)
    src/ops/arith/wrapping.rs - ops::arith::wrapping::WrappingAddAssign::wrapping_add_assign (line 64)
test result: FAILED. 2813 passed; 2 failed; 27 ignored; 0 measured; 0 filtered out; finished in 40.24s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:35
