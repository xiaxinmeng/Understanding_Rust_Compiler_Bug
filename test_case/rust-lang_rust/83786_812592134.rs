plain
.................................................................................................... 9400/11732
.................................................................................................... 9500/11732
.........................................................................i.......i.................. 9600/11732
.................................................................................................... 9700/11732
...................iiiiiii..iiiiii.i................................................................ 9800/11732
.................................................................................................... 10000/11732
.................................................................................................... 10100/11732
.................................................................................................... 10200/11732
.................................................................................................... 10300/11732
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.086 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.31s

 finished in 2.366 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

   Doc-tests core

running 2847 tests
iiiiii.................................................FF........................................... 100/2847
....ii.............................................................................................. 300/2847
......................................................i............................................. 400/2847
.................................................................................................... 500/2847
...........................i..i...................iiii.............................................. 600/2847
---
.....................i.....................i.....................i.................................. 2800/2847
...............................................
failures:

---- src/cell.rs - cell::RefCell<T>::drop_leak (line 1041) stdout ----
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
11 | c.drop_leak();
   | ^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.
Couldn't compile the test.
Couldn't compile the test.
---- src/cell.rs - cell::RefCell<T>::drop_leak_mut (line 1064) stdout ----
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
11 | c.drop_leak_mut();
   | ^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:42
