plain
.................................................................................................... 9400/11715
.................................................................................................... 9500/11715
........................................................i......i.................................... 9600/11715
.................................................................................................... 9700/11715
..iiiiiii..iiiiii.i................................................................................. 9800/11715
.................................................................................................... 10000/11715
.................................................................................................... 10100/11715
.................................................................................................... 10200/11715
.................................................................................................... 10300/11715
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.05s

 finished in 0.110 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i...i.i.....ii..ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.440 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

   Doc-tests rustc_lint_defs

running 90 tests
ii.i.....i.......ii.i.............iii.......................iF............i...............

---- src/builtin.rs - builtin::OR_PATTERNS_BACK_COMPAT (line 3188) stdout ----
---- src/builtin.rs - builtin::OR_PATTERNS_BACK_COMPAT (line 3188) stdout ----
Test compiled successfully, but it's marked `compile_fail`.
failures:
    src/builtin.rs - builtin::OR_PATTERNS_BACK_COMPAT (line 3188)

test result: FAILED. 77 passed; 1 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.69s
test result: FAILED. 77 passed; 1 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.69s

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:22:35
