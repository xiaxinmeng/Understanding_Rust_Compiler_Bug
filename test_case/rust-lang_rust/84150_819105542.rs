plain
.................................................................................................... 9400/11755
.................................................................................................... 9500/11755
.........................................................................................i......i... 9600/11755
.................................................................................................... 9700/11755
...................................iiiiiii..iiiiii.i................................................ 9800/11755
.................................................................................................... 10000/11755
.................................................................................................... 10100/11755
.................................................................................................... 10200/11755
.................................................................................................... 10300/11755
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.169 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.35s

 finished in 2.420 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"src/tools/lint-docs"}) not skipped for "bootstrap::test::LintDocs" -- not in ["src/tools/tidy"]
 finished in 0.932 seconds
Generating lint docs (x86_64-unknown-linux-gnu)
Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
unable to find array "itemTypes"


command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.53.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:29:26
