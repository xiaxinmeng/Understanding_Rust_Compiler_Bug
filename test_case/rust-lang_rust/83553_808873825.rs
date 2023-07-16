plain
.................................................................................................... 9400/11718
.................................................................................................... 9500/11718
............................................................i......i................................ 9600/11718
.................................................................................................... 9700/11718
......iiiiiii..iiiiii.i............................................................................. 9800/11718
.................................................................................................... 10000/11718
.................................................................................................... 10100/11718
.................................................................................................... 10200/11718
.................................................................................................... 10300/11718
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.115 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.396 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
...................................................................................iiii............. 1100/1151
...................................................
failures:

---- src/primitive_docs.rs - prim_pointer (line 456) stdout ----
error: `#[derive]` can't be used on a `#[repr(packed)]` struct that does not derive Copy (error E0133)
  |
4 | #[derive(Debug, Default)]
  |          ^^^^^
  |
  |
note: the lint level is defined here
 --> src/primitive_docs.rs:454:9
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unaligned_references)]` implied by `#[deny(warnings)]`
  = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
  = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:03
