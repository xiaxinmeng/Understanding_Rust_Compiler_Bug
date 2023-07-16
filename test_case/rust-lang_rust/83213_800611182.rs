plain
.................................................................................................... 9300/11682
.................................................................................................... 9400/11682
.................................................................................................... 9500/11682
.........................i......i................................................................... 9600/11682
.......................................................................iiiiiii..iiiiii.i............ 9700/11682
.................................................................................................... 9900/11682
.................................................................................................... 10000/11682
.................................................................................................... 10100/11682
.................................................................................................... 10200/11682
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.00s

 finished in 0.057 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii...........iiii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 1.81s

 finished in 1.863 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 996 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0783 (line 16023) stdout ----
error[E0765]: unterminated double quote string
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16028:47
  |
5 |           _ => println!("Got a number 10 or more")
6 | |     }
7 | | }
  | |_^


error: aborting due to previous error

For more information about this error, try `rustc --explain E0765`.
Some expected error codes were not found: ["E0783"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0782 (line 16005) stdout ----
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16007:18
  |
  |
4 | fn test(arg: Box<Foo>) {}
  |                  ^^^ help: use `dyn`: `dyn Foo`
  = note: `#[warn(bare_trait_objects)]` on by default

warning: 1 warning emitted


Test compiled successfully, but it's marked `compile_fail`.
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0782 (line 16005)
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0783 (line 16023)

