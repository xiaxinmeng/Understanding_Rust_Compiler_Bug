plain
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
---
Checking "typed-query" ... OK
Checking "vec-new" ... OK


command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.55.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"


Build completed unsuccessfully in 0:23:27
