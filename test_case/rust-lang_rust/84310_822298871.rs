plain
.................................................................................................... 9400/11765
.................................................................................................... 9500/11765
................................................................................................i... 9600/11765
...i................................................................................................ 9700/11765
..........................................iiiiiii..iiiiii..i........................................ 9800/11765
.................................................................................................... 10000/11765
.................................................................................................... 10100/11765
.................................................................................................... 10200/11765
.................................................................................................... 10300/11765
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.181 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i..i.i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.35s

 finished in 2.415 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 994 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0779 (line 15947) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0723 (line 14429) stdout ----
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:14430:14
  |
3 | const fn foo<T: Copy>(_: T) { // error!
  |
  = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
  = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable


error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Some expected error codes were not found: ["E0723"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0723 (line 14438) stdout ----
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:14441:14
  |
5 | const fn foo<T: Copy>(_: T) { // ok!
  |
  = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
  = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable

