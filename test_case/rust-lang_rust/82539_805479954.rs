plain
.................................................................................................... 9300/11704
.................................................................................................... 9400/11704
.................................................................................................... 9500/11704
.............................................i......i............................................... 9600/11704
...........................................................................................iiiiiii.. 9700/11704
iiiiii.i............................................................................................ 9800/11704
.................................................................................................... 10000/11704
.................................................................................................... 10100/11704
.................................................................................................... 10200/11704
.................................................................................................... 10300/11704
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
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.21s

 finished in 2.285 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
..................................................................................................
test result: ok. 298 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

   Doc-tests core
error: `#[doc(include)]` is deprecated and will be removed in a future release
   |
   |
12 | #[doc(include = "core_arch_docs.md")]
   |
   |
   = note: `-D rustdoc::doc-include` implied by `-D warnings`
   = note: for more information, see issue #44732 <https://github.com/rust-lang/rust/issues/44732>
   = note: for more information, see issue #44732 <https://github.com/rust-lang/rust/issues/44732>
help: use `#![feature(extended_key_value_attributes)]` instead
   |
12 | #[doc = include_str!("core_arch_docs.md")]

error: aborting due to previous error

error: test failed, to rerun pass '--doc'
error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:22
