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

 finished in 0.067 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 9.716 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.34s

 finished in 2.415 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1200/1251
...................................................
failures:

---- nonzero::test_nonzero_checked_mul stdout ----
thread 'nonzero::test_nonzero_checked_mul' panicked at 'assertion failed: `(left == right)`
  left: `Some(4294967295)`,
 right: `Some(4294967294)`', library/core/tests/nonzero.rs:355:5

failures:
    nonzero::test_nonzero_checked_mul


test result: FAILED. 1248 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 5.81s

error: test failed, to rerun pass '-p core --test coretests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:36
