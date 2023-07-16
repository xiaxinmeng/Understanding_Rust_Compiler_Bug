plain
Checking "alias-2" ... OK
Checking "alias-3" ... OK
Checking "alias" ... OK
Checking "basic" ... FAILED
==> Result not found in 'in_args': '{"path":"std::str","name":"eq"}'
Diff of first error:
 {
-     "path": "std::str",
+     "path": "proc_macro",
-     "name": "eq",
+     "name": "new",
 }
==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
+     "path": "proc_macro::tracked_env",
-     "name": "add",
+     "name": "var",
Checking "deduplication" ... OK
Checking "enum-option" ... OK
Checking "filter-crate" ... OK
Checking "fn-forget" ... OK
---
Checking "never" ... OK
Checking "primitive" ... OK
Checking "quoted" ... OK
Checking "return-specific-literal" ... FAILED
==> Result not found in 'in_args': '{"path":"std::string::String","name":"ne"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "proc_macro",
+     "path": "proc_macro",
-     "name": "ne",
+     "name": "new",
 }
==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
+     "path": "proc_macro::tracked_env",
-     "name": "add",
+     "name": "var",
Checking "return-specific" ... FAILED
Checking "return-specific" ... FAILED
==> Result not found in 'in_args': '{"path":"std::string::String","name":"ne"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "proc_macro",
+     "path": "proc_macro",
-     "name": "ne",
+     "name": "new",
 }
==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
+     "path": "proc_macro::tracked_env",
-     "name": "add",
+     "name": "var",
Checking "should-fail" ... OK
Checking "string-from_ut" ... OK
Checking "struct-vec" ... OK
Checking "vec-new" ... OK
Checking "vec-new" ... OK


command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.55.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:28:48
