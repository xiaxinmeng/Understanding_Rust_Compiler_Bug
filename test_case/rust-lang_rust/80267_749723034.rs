plain
.................................................................................................... 9000/11196
.................................................................................................... 9100/11196
......................................................................................i......i...... 9200/11196
.................................................................................................... 9300/11196
.........................iiiiii..iiiiii.i........................................................... 9400/11196
.................................................................................................... 9600/11196
.................................................................................................... 9700/11196
.......................................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
............................. 9800/11196
---
 finished in 0.450 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.078 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i.....ii.i.i....ii..........iiii.........i.....i...i.......ii.i..ii....iiii...... 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.469 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 981 tests
---

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
Some expected error codes were not found: ["E0780"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0780 (line 15698)
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0780 (line 15714)

