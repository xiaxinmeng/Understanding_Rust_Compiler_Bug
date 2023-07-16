plain
Some tests failed in compiletest suite=rustdoc-js mode=js-doc-test host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
............F......
failures:

---- [js-doc-test] src/test/rustdoc-js/generics-impl.rs stdout ----
error: rustdoc-js test failed!
status: exit status: 1
status: exit status: 1
command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js/generics-impl" "--crate-name" "generics_impl" "--test-file" "/checkout/src/test/rustdoc-js/generics-impl.js"
Testing /checkout/src/test/rustdoc-js/generics-impl.js ... FAILED
Testing /checkout/src/test/rustdoc-js/generics-impl.js ... FAILED
[ query `-> Ddddddd`]==> Expected exactly 1 results but found 0 in 'returned'
[ query `-> Ddddddd`]==> Result not found in 'returned': '{"path":"generics_impl::Ddddddd","name":"hhhhhhh"}'
Diff of first error:
 {
-     "path": "generics_impl::Ddddddd",
-     "name": "hhhhhhh",
------------------------------------------
stderr: none


