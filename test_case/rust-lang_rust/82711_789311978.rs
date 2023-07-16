plain
.................................................................................................... 9300/11524
.................................................................................................... 9400/11524
.........................................................................i......i................... 9500/11524
.................................................................................................... 9600/11524
............iiiiiii..iiiiii.i....................................................................... 9700/11524
.................................................................................................... 9900/11524
.................................................................................................... 10000/11524
.................................................................................................... 10100/11524
.................................................................................................... 10200/11524
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.083 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.25s

 finished in 2.324 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- src/string.rs - string::Cow::from (line 2379) stdout ----
error[E0283]: type annotations needed
 --> src/string.rs:2383:26
  |
7 | assert_eq!(Cow::from(s), Cow::Owned(s2));
  |                          ^^^^^^^^^^ cannot infer type for type parameter `B` declared on the enum `Cow`
  |
  = note: cannot satisfy `_: ToOwned`
  = note: required by `Owned`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:31
