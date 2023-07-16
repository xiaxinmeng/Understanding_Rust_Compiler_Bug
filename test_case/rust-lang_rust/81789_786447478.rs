plain
.................................................................................................... 9200/11502
.................................................................................................... 9300/11502
.................................................................................................... 9400/11502
......................................................i......i...................................... 9500/11502
.............................................................................................iiiiiii 9600/11502
..iiiiii.i.......................................................................................... 9700/11502
.................................................................................................... 9900/11502
.................................................................................................... 10000/11502
.................................................................................................... 10100/11502
.................................................................................................... 10200/11502
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.079 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.34s

 finished in 2.421 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- src/builtin.rs - builtin::INVALID_PTR_TO_INT_CAST (line 2973) stdout ----
warning: unknown lint: `ptr_to_int_cast`
 --> src/builtin.rs:2974:9
  |
2 | #![deny(ptr_to_int_cast)]
  |
  = note: `#[warn(unknown_lints)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted

Test compiled successfully, but it's marked `compile_fail`.
failures:
    src/builtin.rs - builtin::INVALID_PTR_TO_INT_CAST (line 2973)

test result: FAILED. 76 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out; finished in 0.75s
test result: FAILED. 76 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out; finished in 0.75s

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:56
