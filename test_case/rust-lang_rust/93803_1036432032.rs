plain
Suite(test::src/test/rustdoc-js) not skipped for "bootstrap::test::RustdocJSNotStd" -- not in [src/tools/tidy]
Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 15 tests
Some tests failed in compiletest suite=rustdoc-js mode=js-doc-test host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..F.F......F...

---- [js-doc-test] rustdoc-js/generics-multi-trait.rs stdout ----

error: rustdoc-js test failed!
error: rustdoc-js test failed!
status: exit status: 1
command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js/generics-multi-trait" "--crate-name" "generics_multi_trait" "--test-file" "/checkout/src/test/rustdoc-js/generics-multi-trait.js"
------------------------------------------
------------------------------------------
Checking "generics-multi-trait" ... FAILED
[ query `Zzzzzzzzzzzzzzzzzz`]==> Expected exactly 1 results but found 0 in 'in_args'
[ query `Zzzzzzzzzzzzzzzzzz`]==> Result not found in 'in_args': '{"path":"generics_multi_trait","name":"beta"}'
Diff of first error:
 {
-     "path": "generics_multi_trait",
-     "name": "beta",
 }
[ query `Zzzzzzzzzzzzzzzzzz`]==> Expected exactly 1 results but found 0 in 'returned'
[ query `Zzzzzzzzzzzzzzzzzz`]==> Result not found in 'returned': '{"path":"generics_multi_trait","name":"bet"}'
Diff of first error:
 {
-     "path": "generics_multi_trait",
-     "name": "bet",

------------------------------------------
stderr:
------------------------------------------
---
---- [js-doc-test] rustdoc-js/generics.rs stdout ----

error: rustdoc-js test failed!
status: exit status: 1
command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js/generics" "--crate-name" "generics" "--test-file" "/checkout/src/test/rustdoc-js/generics.js"
------------------------------------------
------------------------------------------
Checking "generics" ... FAILED
[ query `TraitCat`]==> Expected exactly 1 results but found 0 in 'in_args'
[ query `TraitCat`]==> Result not found in 'in_args': '{"path":"generics","name":"gamma"}'
Diff of first error:
 {
-     "path": "generics",
-     "name": "gamma",

------------------------------------------
stderr:
------------------------------------------
---
---- [js-doc-test] rustdoc-js/generics-trait.rs stdout ----

error: rustdoc-js test failed!
status: exit status: 1
command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js/generics-trait" "--crate-name" "generics_trait" "--test-file" "/checkout/src/test/rustdoc-js/generics-trait.js"
------------------------------------------
------------------------------------------
Checking "generics-trait" ... FAILED
[ query `OtherThingxxxxxxxx`]==> Result not found in 'in_args': '{"path":"generics_trait","name":"alpha"}'
Diff of first error:
 {
-     "path": "generics_trait",
-     "name": "alpha",
 }
[ query `OtherThingxxxxxxxx`]==> Result not found in 'returned': '{"path":"generics_trait","name":"alef"}'
Diff of first error:
 {
-     "path": "generics_trait",
-     "name": "alef",

------------------------------------------
stderr:
------------------------------------------
