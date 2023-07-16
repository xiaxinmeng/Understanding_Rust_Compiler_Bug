plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]

 finished in 0.103 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i...i.i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii......iiii....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.22s

 finished in 2.286 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1200/1248
................................................
failures:

---- num::from_str_issue7588 stdout ----
thread 'num::from_str_issue7588' panicked at 'assertion failed: `(left == right)`
  left: `Some(232)`,
 right: `None`', library/core/tests/num/mod.rs:87:5
---- num::test_invalid stdout ----
---- num::test_invalid stdout ----
thread 'num::test_invalid' panicked at 'assertion failed: `(left == right)`
  left: `Err(PosOverflow)`,
 right: `Err(InvalidDigit)`', library/core/tests/num/mod.rs:81:5

failures:
error: test failed, to rerun pass '-p core --test coretests'
    num::from_str_issue7588
    num::from_str_issue7588
    num::test_invalid

test result: FAILED. 1244 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out; finished in 5.51s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:47
