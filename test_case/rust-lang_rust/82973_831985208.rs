plain
.................................................................................................... 9400/11823
.................................................................................................... 9500/11823
.................................................................................................... 9600/11823
...........................................i......i................................................. 9700/11823
.........................................................................................iiiiiii...i 9800/11823
iiiiii.............................................................................................. 9900/11823
.................................................................................................... 10100/11823
.................................................................................................... 10200/11823
.................................................................................................... 10300/11823
.................................................................................................... 10400/11823
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.170 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.401 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
..........................................................................................iiii...... 1100/1158
..........................................................
failures:

---- src/process.rs - process::ExitStatus::exit_ok (line 1409) stdout ----
error[E0658]: use of unstable library feature 'exit_status_error'
   |
   |
13 | status.exit_ok().expect_err("/dev/nonexistent could be listed!");
   |
   = note: see issue #84908 <https://github.com/rust-lang/rust/issues/84908> for more information
   = help: add `#![feature(exit_status_error)]` to the crate attributes to enable

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:07
