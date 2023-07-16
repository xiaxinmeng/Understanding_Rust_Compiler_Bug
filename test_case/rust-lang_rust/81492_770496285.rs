plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.065 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 9.725 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.29s

 finished in 2.365 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Building stage1 tool error_index_generator (x86_64-unknown-linux-gnu)
   Compiling same-file v1.0.6
   Compiling walkdir v2.3.1
   Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
error[E0603]: module `html` is private
   |
   |
17 | use rustdoc::html::markdown::{ErrorCodes, IdMap, Markdown, Playground};
   |              ^^^^ private module
   |
note: the module `html` is defined here
   |
   |
87 | mod html;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.
