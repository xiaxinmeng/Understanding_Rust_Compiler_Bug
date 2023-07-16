plain
.................................................................................................... 9400/11819
.................................................................................................... 9500/11819
.................................................................................................... 9600/11819
........................................i......i.................................................... 9700/11819
......................................................................................iiiiiii..iiiii 9800/11819
.................................................................................................... 10000/11819
.................................................................................................... 10100/11819
.................................................................................................... 10200/11819
.................................................................................................... 10300/11819
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.169 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii.....ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.41s

 finished in 2.472 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1200/1268
....................................................................
failures:

---- nonzero::test_from_str stdout ----
thread 'nonzero::test_from_str' panicked at 'assertion failed: `(left == right)`
  left: `Some(PosOverflow)`,
 right: `Some(NegOverflow)`', library/core/tests/nonzero.rs:139:5
---- num::test_int_from_str_overflow stdout ----
---- num::test_int_from_str_overflow stdout ----
thread 'num::test_int_from_str_overflow' panicked at 'assertion failed: `(left == right)`
  left: `Err(PosOverflow)`,
 right: `Err(NegOverflow)`', library/core/tests/num/mod.rs:82:5

failures:
    nonzero::test_from_str
    num::test_int_from_str_overflow
    num::test_int_from_str_overflow

test result: FAILED. 1264 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out; finished in 5.52s

error: test failed, to rerun pass '-p core --test coretests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:47
