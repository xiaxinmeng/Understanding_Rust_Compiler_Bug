plain
.................................................................................................... 9400/11736
.................................................................................................... 9500/11736
............................................................................i......i................ 9600/11736
.................................................................................................... 9700/11736
......................iiiiiii..iiiiii.i............................................................. 9800/11736
.................................................................................................... 10000/11736
.................................................................................................... 10100/11736
.................................................................................................... 10200/11736
.................................................................................................... 10300/11736
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.091 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.27s

 finished in 2.335 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
...................................................................................................
test result: ok. 299 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

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
Build completed unsuccessfully in 0:17:43
