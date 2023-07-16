plain
.................................................................................................... 9200/11494
.................................................................................................... 9300/11494
.................................................................................................... 9400/11494
..................................................i......i.......................................... 9500/11494
.........................................................................................iiiiiii..ii 9600/11494
.................................................................................................... 9800/11494
.................................................................................................... 9900/11494
.................................................................................................... 10000/11494
.................................................................................................... 10100/11494
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.078 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.31s

 finished in 2.391 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- src/builtin.rs - builtin::PTR_TO_INT_CAST (line 2973) stdout ----
error: casting pointer to u32
 --> src/builtin.rs:2978:13
  |
6 |     let y = u16::max as u32; // a function pointer, cast to u32, causing an overflow
  |             |
  |             casting pointer to u32
  |             casting pointer to u32
  |             It only makes sense to cast a pointer to `usize`.
  |             If you really want to cast to `u32`, try `_ as usize as u32`
note: the lint level is defined here
 --> src/builtin.rs:2974:9
  |
  |
2 | #![deny(ptr_to_int_cast)]

error: aborting due to previous error

Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:20
