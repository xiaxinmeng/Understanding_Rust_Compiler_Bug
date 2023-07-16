plain
 finished in 0.918 seconds
Generating lint docs (x86_64-unknown-linux-gnu)
Suite(test::src/test/rustdoc-js-std) not skipped for "bootstrap::test::RustdocJSStd" -- not in [src/tools/tidy]
Checking "alias-1" ... FAILED
==> Result not found in 'others': '{"path":"std","name":"reference"}'
Diff of first error:
-     "path": "std",
-     "name": "reference",
 }
Checking "alias-2" ... FAILED
Checking "alias-2" ... FAILED
==> Result not found in 'others': '{"path":"std::ops","name":"AddAssign"}'
Diff of first error:
 {
-     "path": "std::ops",
-     "name": "AddAssign",
 }
==> Result not found in 'others': '{"path":"std::ops","name":"Add"}'
Diff of first error:
 {
-     "path": "std::ops",
-     "name": "Add",
 }
==> Result not found in 'others': '{"path":"core::ops","name":"AddAssign"}'
Diff of first error:
 {
-     "path": "core::ops",
-     "name": "AddAssign",
 }
==> Result not found in 'others': '{"path":"core::ops","name":"Add"}'
Diff of first error:
 {
-     "path": "core::ops",
-     "name": "Add",
Checking "alias-3" ... FAILED
Checking "alias-3" ... FAILED
==> Result not found in 'others': '{"path":"std","name":"never"}'
Diff of first error:
-     "path": "std",
-     "name": "never",
 }
Checking "alias-4" ... FAILED
Checking "alias-4" ... FAILED
==> Result not found in 'others': '{"name":"Ord"}'
Diff of first error:
 {
-     "name": "Ord",
Checking "alias" ... FAILED
Checking "alias" ... FAILED
==> Result not found in 'others': '{"path":"std","name":"slice"}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "slice",
 }
==> Result not found in 'others': '{"path":"std::ops","name":"IndexMut"}'
Diff of first error:
 {
-     "path": "std::ops",
-     "name": "IndexMut",
 }
==> Result not found in 'others': '{"path":"std::ops","name":"Index"}'
Diff of first error:
 {
-     "path": "std::ops",
-     "name": "Index",
Checking "basic" ... FAILED
Checking "basic" ... FAILED
==> Result not found in 'others': '{"path":"std::string","name":"String"}'
Diff of first error:
-     "path": "std::string",
-     "name": "String",
 }
 }
==> Result not found in 'others': '{"path":"std::ffi","name":"CString"}'
Diff of first error:
 {
-     "path": "std::ffi",
-     "name": "CString",
 }
==> Result not found in 'others': '{"path":"std::ffi","name":"OsString"}'
Diff of first error:
 {
-     "path": "std::ffi",
-     "name": "OsString",
 }
==> Result not found in 'in_args': '{"path":"std::str","name":"eq"}'
Diff of first error:
 {
-     "path": "std::str",
-     "name": "eq",
 }
==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "add",
Checking "deduplication" ... FAILED
Checking "deduplication" ... FAILED
==> Result not found in 'others': '{"path":"std::f32","name":"is_nan"}'
Diff of first error:
 {
-     "path": "std::f32",
-     "name": "is_nan",
 }
==> Result not found in 'others': '{"path":"std::f64","name":"is_nan"}'
Diff of first error:
 {
-     "path": "std::f64",
-     "name": "is_nan",
 }
==> Result not found in 'others': '{"path":"std::option::Option","name":"is_none"}'
Diff of first error:
-     "path": "std::option::Option",
-     "path": "std::option::Option",
-     "name": "is_none",
Checking "enum-option" ... FAILED
Checking "enum-option" ... FAILED
==> Result not found in 'others': '{"path":"std::option","name":"Option"}'
Diff of first error:
-     "path": "std::option",
-     "path": "std::option",
-     "name": "Option",
Checking "filter-crate" ... OK
Checking "fn-forget" ... FAILED
Checking "fn-forget" ... FAILED
==> Result not found in 'others': '{"path":"std::mem","name":"forget"}'
Diff of first error:
 {
-     "path": "std::mem",
-     "name": "forget",
 }
==> Result not found in 'others': '{"path":"std::fmt","name":"format"}'
Diff of first error:
 {
-     "path": "std::fmt",
-     "name": "format",
Checking "from_u" ... FAILED
Checking "from_u" ... FAILED
==> Result not found in 'others': '{"path":"std::char","name":"from_u32"}'
Diff of first error:
 {
-     "path": "std::char",
-     "name": "from_u32",
 }
==> Result not found in 'others': '{"path":"std::str","name":"from_utf8"}'
Diff of first error:
 {
-     "path": "std::str",
-     "name": "from_utf8",
 }
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "from_utf8",
Checking "keyword" ... FAILED
Checking "keyword" ... FAILED
==> Result not found in 'others': '{"path":"std","name":"fn","ty":15}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "fn",
-     "ty": "15",
 }
==> Result not found in 'others': '{"path":"std","name":"fn","ty":21}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "fn",
-     "ty": "21",
Checking "macro-check" ... FAILED
Checking "macro-check" ... FAILED
==> Result not found in 'others': '{"path":"std","name":"panic","ty":14}'
Diff of first error:
-     "path": "std",
-     "name": "panic",
-     "name": "panic",
-     "ty": "14",
 }
==> Result not found in 'others': '{"path":"std","name":"panic","ty":0}'
Diff of first error:
-     "path": "std",
-     "name": "panic",
-     "name": "panic",
-     "ty": "0",
Checking "macro-print" ... FAILED
Checking "macro-print" ... FAILED
==> Result not found in 'others': '{"path":"std","name":"print"}'
Diff of first error:
-     "path": "std",
-     "name": "print",
 }
 }
==> Result not found in 'others': '{"path":"std","name":"eprint"}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "eprint",
 }
==> Result not found in 'others': '{"path":"std","name":"println"}'
Diff of first error:
-     "path": "std",
-     "name": "println",
 }
 }
==> Result not found in 'others': '{"path":"std","name":"eprintln"}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "eprintln",
Checking "multi-query" ... FAILED
Checking "multi-query" ... FAILED
==> Result not found in 'others': '{"path":"std","name":"str","href":"../std/primitive.str.html"}'
Diff of first error:
-     "path": "std",
-     "name": "str",
-     "name": "str",
-     "href": "../std/primitive.str.html",
 }
==> Result not found in 'others': '{"path":"std","name":"u8","href":"../std/primitive.u8.html"}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "u8",
-     "href": "../std/primitive.u8.html",
 }
==> Result not found in 'others': '{"path":"std","name":"str","href":"../std/str/index.html"}'
Diff of first error:
-     "path": "std",
-     "name": "str",
-     "name": "str",
-     "href": "../std/str/index.html",
 }
==> Result not found in 'others': '{"path":"std","name":"u8","href":"../std/u8/index.html"}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "u8",
-     "href": "../std/u8/index.html",
Checking "never" ... FAILED
Checking "never" ... FAILED
==> Result not found in 'others': '{"path":"std","name":"never"}'
Diff of first error:
-     "path": "std",
-     "name": "never",
 }
Checking "primitive" ... FAILED
Checking "primitive" ... FAILED
[ query `i8`]==> Result not found in 'others': '{"path":"std","name":"i8","href":"../std/primitive.i8.html"}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "i8",
-     "href": "../std/primitive.i8.html",
Checking "quoted" ... FAILED
Checking "quoted" ... FAILED
==> Result not found in 'others': '{"path":"std","name":"error"}'
Diff of first error:
-     "path": "std",
-     "name": "error",
 }
 }
==> Result not found in 'others': '{"path":"std::fmt","name":"Error"}'
Diff of first error:
 {
-     "path": "std::fmt",
-     "name": "Error",
 }
==> Result not found in 'others': '{"path":"std::io","name":"Error"}'
Diff of first error:
 {
-     "path": "std::io",
-     "name": "Error",
 }
==> Result not found in 'returned': '{"path":"std::fmt::LowerExp","name":"fmt"}'
Diff of first error:
 {
-     "path": "std::fmt::LowerExp",
-     "name": "fmt",
Checking "return-specific-literal" ... FAILED
Checking "return-specific-literal" ... FAILED
==> Result not found in 'in_args': '{"path":"std::string::String","name":"ne"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "ne",
 }
==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "add",
Checking "return-specific" ... FAILED
Checking "return-specific" ... FAILED
==> Result not found in 'in_args': '{"path":"std::string::String","name":"ne"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "ne",
 }
==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "add",
Checking "should-fail" ... OK
Checking "string-from_ut" ... FAILED
Checking "string-from_ut" ... FAILED
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "from_utf8",
 }
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "from_utf8",
 }
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8_lossy"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "from_utf8_lossy",
 }
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf16_lossy"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "from_utf16_lossy",
 }
==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8_unchecked"}'
Diff of first error:
-     "path": "std::string::String",
-     "path": "std::string::String",
-     "name": "from_utf8_unchecked",
Checking "struct-vec" ... FAILED
Checking "struct-vec" ... FAILED
==> Result not found in 'others': '{"path":"std::vec","name":"Vec"}'
Diff of first error:
 {
-     "path": "std::vec",
-     "name": "Vec",
 }
==> Result not found in 'others': '{"path":"std::collections","name":"VecDeque"}'
Diff of first error:
 {
-     "path": "std::collections",
-     "name": "VecDeque",
Checking "typed-query" ... FAILED
Checking "typed-query" ... FAILED
==> Expected exactly 4 results but found 0 in 'others'
==> Result not found in 'others': '{"path":"std","name":"print"}'
Diff of first error:
-     "path": "std",
-     "name": "print",
 }
 }
==> Result not found in 'others': '{"path":"std","name":"eprint"}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "eprint",
 }
==> Result not found in 'others': '{"path":"std","name":"println"}'
Diff of first error:
-     "path": "std",
-     "name": "println",
 }
 }
==> Result not found in 'others': '{"path":"std","name":"eprintln"}'
Diff of first error:
-     "path": "std",
-     "path": "std",
-     "name": "eprintln",
Checking "vec-new" ... FAILED
Checking "vec-new" ... FAILED
==> Result not found in 'others': '{"path":"std::vec::Vec","name":"new"}'
Diff of first error:
 {
-     "path": "std::vec::Vec",
-     "name": "new",
 }
==> Result not found in 'others': '{"path":"std::vec::Vec","name":"ne"}'
Diff of first error:
 {
-     "path": "std::vec::Vec",
-     "name": "ne",
 }
==> Result not found in 'others': '{"path":"std::rc::Rc","name":"ne"}'
Diff of first error:
 {
-     "path": "std::rc::Rc",
-     "name": "ne",
Build completed unsuccessfully in 0:28:04
