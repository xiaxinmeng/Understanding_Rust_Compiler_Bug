plain
.................................................................................................... 9400/11805
.................................................................................................... 9500/11805
.................................................................................................... 9600/11805
...........................i......i................................................................. 9700/11805
.........................................................................iiiiiii..iiiiii.i.......... 9800/11805
.................................................................................................... 10000/11805
.................................................................................................... 10100/11805
.................................................................................................... 10200/11805
.................................................................................................... 10300/11805
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.168 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.387 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
......................F..............................................................i.iii.......... 1100/1153
.....................................................
failures:

---- src/sys/unix/ext/fs.rs - sys::unix::ext::fs::chroot (line 897) stdout ----
error[E0658]: use of unstable library feature 'unix_chroot'
  |
  |
6 |     fs::chroot("/sandbox")?;
  |
  = note: see issue #84715 <https://github.com/rust-lang/rust/issues/84715> for more information
  = note: see issue #84715 <https://github.com/rust-lang/rust/issues/84715> for more information
  = help: add `#![feature(unix_chroot)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:51
