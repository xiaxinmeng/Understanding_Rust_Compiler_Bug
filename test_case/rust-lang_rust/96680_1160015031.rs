plain
---- [js-doc-test] src/test/rustdoc-js/generics-impl.rs stdout ----

error: rustdoc-js test failed!
status: exit status: 1
command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js/generics-impl" "--crate-name" "generics_impl" "--test-file" "/checkout/src/test/rustdoc-js/generics-impl.js"
Testing /checkout/src/test/rustdoc-js/generics-impl.js ... FAILED
Testing /checkout/src/test/rustdoc-js/generics-impl.js ... FAILED
[ query `Read -> u64`]==> Expected exactly 2 results but found 0 in 'others'
[ query `Read -> u64`]==> Result not found in 'others': '{"path":"generics_impl::Ddddddd","name":"eeeeeee"}'
Diff of first error:
 {
-     "path": "generics_impl::Ddddddd",
-     "name": "eeeeeee",
 }
[ query `Read -> u64`]==> Result not found in 'others': '{"path":"generics_impl::Ddddddd","name":"ggggggg"}'
Diff of first error:
 {
-     "path": "generics_impl::Ddddddd",
-     "name": "ggggggg",
------------------------------------------
stderr: none


