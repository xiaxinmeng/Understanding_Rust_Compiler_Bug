plain
.................................................................................................... 9400/11716
.................................................................................................... 9500/11716
.........................................................i......i................................... 9600/11716
.................................................................................................... 9700/11716
...iiiiiii..iiiiii.i................................................................................ 9800/11716
.................................................................................................... 10000/11716
.................................................................................................... 10100/11716
.................................................................................................... 10200/11716
.................................................................................................... 10300/11716
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.116 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 12.076 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.45s

 finished in 2.518 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
running 90 tests
ii.i.....i.......ii.i.............ii.i....................F.i.............i...............
failures:

---- src/builtin.rs - builtin::OR_PATTERNS_BACK_COMPAT (line 3188) stdout ----
warning: detects usage of old versions of or patterns
  |
  |
4 |     ( $expr:expr , $( $( $pat:pat )|+ => $expr_arm:expr ),+ ) => {
  |
note: the lint level is defined here
 --> src/builtin.rs:3189:9
  |
  |
2 | #![warn(or_patterns_back_compat)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^
  = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.
warning: 1 warning emitted


Test compiled successfully, but it's marked `compile_fail`.
failures:
    src/builtin.rs - builtin::OR_PATTERNS_BACK_COMPAT (line 3188)

test result: FAILED. 77 passed; 1 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.70s
test result: FAILED. 77 passed; 1 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.70s

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:24:21
