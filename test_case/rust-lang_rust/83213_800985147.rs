plain
.................................................................................................... 9300/11692
.................................................................................................... 9400/11692
.................................................................................................... 9500/11692
...................................i......i......................................................... 9600/11692
.................................................................................iiiiiii..iiiiii.i.. 9700/11692
.................................................................................................... 9900/11692
.................................................................................................... 10000/11692
.................................................................................................... 10100/11692
.................................................................................................... 10200/11692
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]

 finished in 0.112 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.46s

 finished in 2.526 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 996 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0777 (line 15883) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0783 (line 16023) stdout ----
error: range-to patterns with `...` are not allowed
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16027:9
  |
5 |         ...9 => println!("Got a number less than 10"),
  |         ^^^ help: use `..=` instead

error[E0658]: half-open range patterns are unstable
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16027:9
  |
5 |         ...9 => println!("Got a number less than 10"),
  |
  = note: see issue #67264 <https://github.com/rust-lang/rust/issues/67264> for more information
  = note: see issue #67264 <https://github.com/rust-lang/rust/issues/67264> for more information
  = help: add `#![feature(half_open_range_patterns)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
