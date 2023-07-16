plain
.................................................................................................... 9300/11553
.................................................................................................... 9400/11553
.................................................................................i......i........... 9500/11553
.................................................................................................... 9600/11553
...........................iiiiiii..iiiiii.i........................................................ 9700/11553
.................................................................................................... 9900/11553
.................................................................................................... 10000/11553
.................................................................................................... 10100/11553
.................................................................................................... 10200/11553
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.00s

 finished in 0.077 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i.i....ii...........iiii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.438 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Checking "keyword" ... OK
Checking "macro-check" ... OK
Checking "macro-print" ... OK
Checking "multi-query" ... FAILED
==> Result not found in 'others': '{"path":"std::ffi","name":"CStr"}'
Diff of first error:
 {
-     "path": "std::ffi",
+     "path": "std",
-     "name": "CStr",
+     "name": "str",
Checking "never" ... OK
Checking "primitive" ... OK
Checking "quoted" ... OK
Checking "return-specific-literal" ... OK
Checking "return-specific-literal" ... OK
Checking "return-specific" ... OK
Checking "should-fail" ... OK
Checking "string-from_ut" ... OK
Checking "struct-vec" ... FAILED
==> Result not found in 'others': '{"path":"std::collections","name":"VecDeque"}'
Diff of first error:
 {
-     "path": "std::collections",
+     "path": "alloc::vec",
-     "name": "VecDeque",
+     "name": "Vec",
Checking "vec-new" ... OK



command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.52.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:37
