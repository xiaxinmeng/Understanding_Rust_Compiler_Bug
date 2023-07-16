plain
.................................................................................................... 9000/11228
.................................................................................................... 9100/11228
.................................................................................................... 9200/11228
...............i......i............................................................................. 9300/11228
......................................................iiiiii..iiiiii.i.............................. 9400/11228
.................................................................................................... 9600/11228
.................................................................................................... 9700/11228
.................................................................................................... 9800/11228
.................................................................................................... 9900/11228
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 238 tests
iii............iii...ii..i.ii.........i.ii..........i..............i.............i.i...iii.......iii 100/238
..........i.....i.............i.i.i....ii..iiii.......................................ii..i..i...i.. 200/238
test result: ok. 193 passed; 0 failed; 45 ignored; 0 measured; 0 filtered out; finished in 2.74s

 finished in 2.804 seconds
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.063 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i....ii..ii.....ii....ii..........iiii.........i.....i.....i.....ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.29s

 finished in 2.360 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 100/159
.............................F.............................
failures:

---- spec::tests::wasm64_unknown_unknown stdout ----
thread 'spec::tests::wasm64_unknown_unknown' panicked at 'assertion failed: `(left != right)`
  left: `"unknown"`,
 right: `"unknown"`', compiler/rustc_target/src/spec/tests/tests_impl.rs:45:13


failures:
    spec::tests::wasm64_unknown_unknown
    spec::tests::wasm64_unknown_unknown

test result: FAILED. 158 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustc_target --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_target" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:26:35
