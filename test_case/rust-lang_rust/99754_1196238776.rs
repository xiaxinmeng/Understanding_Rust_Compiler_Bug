plain
 finished in 41.172 seconds
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 72 tests
2022-07-27T03:59:57.239657Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// Test for issue 85480\n// Pretty print anonymous struct and union types\n\n// pp-exact\n// pretty-compare-only\n\nstruct Foo {\n    _: union {\n           _: struct {\n                  a: u8,\n                  b: u16,\n              },\n           c: u32,\n       },\n    d: u64,\n    e: f32,\n}\n\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n// Test for issue 85480\n// Pretty print anonymous struct and union types\n\n// pp-exact\n// pretty-compare-only\n\nstruct Foo {\n    _: union  {\n        _: struct  {\n            a: u8,\n            b: u16,\n        },\n        c: u32,\n    },\n    d: u64,\n    e: f32,\n}\n\nfn main() {}\n\n------------------------------------------\ndiff:\n------------------------------------------\n5\t// pretty-compare-only\n6\t\n7\tstruct Foo {\n-\t    _: union {\n-\t           _: struct {\n-\t                  a: u8,\n-\t                  b: u16,\n-\t              },\n-\t           c: u32,\n-\t       },\n+\t    _: union  {\n+\t        _: struct  {\n+\t            a: u8,\n+\t            b: u16,\n+\t        },\n+\t        c: u32,\n+\t    },\n15\t    d: u64,\n16\t    e: f32,\n17\t}\n\n18\t\n-\tfn main() { }\n+\tfn main() {}\n20\t\n\n\n"
Some tests failed in compiletest suite=pretty mode=pretty host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F.......................................................................

---- [pretty] src/test/pretty/anonymous-types.rs stdout ----

error: pretty-printed source does not match expected source
error: pretty-printed source does not match expected source
expected:
------------------------------------------
// Test for issue 85480
// Pretty print anonymous struct and union types

// pp-exact

struct Foo {
    _: union {
           _: struct {
---
------------------------------------------
// Test for issue 85480
// Pretty print anonymous struct and union types

// pp-exact

struct Foo {
    _: union  {
    _: union  {
        _: struct  {
            a: u8,
            b: u16,
        c: u32,
    },
    d: u64,
    e: f32,
---
-                   b: u16,
-               },
-            c: u32,
-        },
+     _: union  {
+         _: struct  {
+             a: u8,
+             b: u16,
+         c: u32,
+     },
15     d: u64,
16     e: f32,
