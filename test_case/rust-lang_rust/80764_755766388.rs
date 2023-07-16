plain
.................................................................................................... 9000/11244
.................................................................................................... 9100/11244
.................................................................................................... 9200/11244
........................................i......i.................................................... 9300/11244
...............................................................................iiiiii..iiiiii.i..... 9400/11244
.................................................................................................... 9600/11244
.................................................................................................... 9700/11244
.................................................................................................... 9800/11244
.................................................................................................... 9900/11244
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.071 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.109 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i.i....ii..........iiii.........i.....i...i........iii.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.432 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
....................................................................F............................... 200/247
..................F............................
failures:

---- rc::tests::test_maybe_thin_unsized stdout ----
thread 'rc::tests::test_maybe_thin_unsized' panicked at 'assertion failed: `(left == right)`
  left: `"\"swordfish\""`,
 right: `"swordfish"`', library/alloc/src/rc/tests.rs:327:5

---- sync::tests::test_maybe_thin_unsized stdout ----
thread 'sync::tests::test_maybe_thin_unsized' panicked at 'assertion failed: `(left == right)`
  left: `"\"swordfish\""`,
 right: `"swordfish"`', library/alloc/src/sync/tests.rs:362:5

failures:
failures:
    rc::tests::test_maybe_thin_unsized
    sync::tests::test_maybe_thin_unsized
test result: FAILED. 245 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s


error: test failed, to rerun pass '-p alloc --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:35
