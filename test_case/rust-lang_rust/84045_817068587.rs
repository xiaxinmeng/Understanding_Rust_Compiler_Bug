plain
.................................................................................................... 9400/11747
.................................................................................................... 9500/11747
...................................................................................i......i......... 9600/11747
.................................................................................................... 9700/11747
.............................iiiiiii..iiiiii..i..................................................... 9800/11747
.................................................................................................... 10000/11747
.................................................................................................... 10100/11747
.................................................................................................... 10200/11747
.................................................................................................... 10300/11747
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.107 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.54s

 finished in 2.609 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 89 tests
ii.i.....i..F....ii.i.............iii......................i.............i...............

---- src/builtin.rs - builtin::ILLEGAL_FLOATING_POINT_LITERAL_PATTERN (line 1684) stdout ----
error: floating-point types cannot be used in patterns
 --> src/builtin.rs:1688:5
 --> src/builtin.rs:1688:5
  |
6 |     5.0 => {}
  |     ^^^
  |
  = note: `#[deny(illegal_floating_point_literal_pattern)]` on by default
  = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>

error: floating-point types cannot be used in patterns
 --> src/builtin.rs:1688:5
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:47
