plain
Checking "alias-3" ... OK
Checking "alias-4" ... OK
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
+     "path": "std::env",
-     "name": "add",
+     "name": "var",
Checking "deduplication" ... FAILED
Checking "deduplication" ... FAILED
==> Result not found in 'others': '{"path":"std::f32","name":"is_nan"}'
Diff of first error:
 {
-     "path": "std::f32",
+     "path": "std",
     "name": "is_nan",
 }
==> Result not found in 'others': '{"path":"std::f64","name":"is_nan"}'
Diff of first error:
 {
-     "path": "std::f64",
+     "path": "std",
     "name": "is_nan",
 }
==> Result not found in 'others': '{"path":"std::option::Option","name":"is_none"}'
Diff of first error:
-     "path": "std::option::Option",
+     "path": "std",
+     "path": "std",
-     "name": "is_none",
+     "name": "is_nan",
Checking "enum-option" ... OK
Checking "filter-crate" ... OK
Checking "fn-forget" ... OK
Checking "from_u" ... FAILED
Checking "from_u" ... FAILED
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
+     "path": "std::str",
     "name": "from_utf8",
Checking "keyword" ... OK
Checking "macro-check" ... OK
Checking "macro-print" ... OK
Checking "multi-query" ... OK
Checking "multi-query" ... OK
Checking "never" ... OK
Checking "primitive" ... OK
Checking "quoted" ... FAILED
==> Result not found in 'returned': '{"path":"std::fmt::LowerExp","name":"fmt"}'
Diff of first error:
 {
-     "path": "std::fmt::LowerExp",
+     "path": "std::fmt",
     "name": "fmt",
Checking "return-specific-literal" ... FAILED
Checking "return-specific-literal" ... FAILED
==> Result not found in 'in_args': '{"path":"std::string::String","name":"ne"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "std",
+     "path": "std",
-     "name": "ne",
+     "name": "eq",
 }
==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "std::string",
+     "path": "std::string",
     "name": "add",
Checking "return-specific" ... FAILED
Checking "return-specific" ... FAILED
==> Result not found in 'in_args': '{"path":"std::string::String","name":"ne"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "std",
+     "path": "std",
-     "name": "ne",
+     "name": "eq",
 }
==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "std::string",
+     "path": "std::string",
     "name": "add",
Checking "should-fail" ... OK
Checking "string-from_ut" ... FAILED
Checking "string-from_ut" ... FAILED
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "std::string",
+     "path": "std::string",
     "name": "from_utf8",
 }
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "std::string",
+     "path": "std::string",
-     "name": "from_utf8",
+     "name": "from_utf16",
 }
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8_lossy"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "std::string",
+     "path": "std::string",
     "name": "from_utf8_lossy",
 }
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf16_lossy"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "std::string",
+     "path": "std::string",
     "name": "from_utf16_lossy",
 }
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8_unchecked"}'
Diff of first error:
-     "path": "std::string::String",
+     "path": "std::string",
+     "path": "std::string",
     "name": "from_utf8_unchecked",
Checking "struct-vec" ... OK
Checking "typed-query" ... OK
Checking "vec-new" ... FAILED
Checking "vec-new" ... FAILED
==> Result not found in 'others': '{"path":"std::vec::Vec","name":"new"}'
Diff of first error:
 {
-     "path": "std::vec::Vec",
+     "path": "std::vec",
     "name": "new",
 }
==> Result not found in 'others': '{"path":"std::vec::Vec","name":"ne"}'
Diff of first error:
 {
-     "path": "std::vec::Vec",
+     "path": "alloc::vec",
-     "name": "ne",
+     "name": "new",
 }
==> Result not found in 'others': '{"path":"std::rc::Rc","name":"ne"}'
Diff of first error:
 {
-     "path": "std::rc::Rc",
+     "path": "std::mem",
-     "name": "ne",
+     "name": "new",



command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.59.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"


Build completed unsuccessfully in 0:35:11
