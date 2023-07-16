plain
.................................................................................................... 9000/11238
.................................................................................................... 9100/11238
.................................................................................................... 9200/11238
..................................i......i.......................................................... 9300/11238
.........................................................................iiiiii..iiiiii.i........... 9400/11238
.................................................................................................... 9600/11238
.................................................................................................... 9700/11238
.................................................................................................... 9800/11238
.................................................................................................... 9900/11238
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.067 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.28s

 finished in 2.349 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 500/541
................F........................
failures:

---- src/vec/mod.rs - vec::Vec<T, A>::leak (line 1757) stdout ----
Test executable failed (terminated by signal).
stderr:
stderr:
free(): double free detected in tcache 2


failures:
    src/vec/mod.rs - vec::Vec<T, A>::leak (line 1757)
    src/vec/mod.rs - vec::Vec<T, A>::leak (line 1757)

test result: FAILED. 539 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 11.83s

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:45
