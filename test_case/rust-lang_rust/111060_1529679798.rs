plain
 finished in 42.327 seconds
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 74 tests
................2023-05-01T12:51:23.309733Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:dollar-crate.pp\n\nfn main() { { ::std::io::_print(format_args!(\"rust\\n\")); }; }\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:dollar-crate.pp\n\nfn main() {\n    { ::std::io::_print(format_args!(\"{0}\\n\", format_args!(\"rust\"))); };\n}\n\n------------------------------------------\ndiff:\n------------------------------------------\n8\t// pretty-mode:expanded\n9\t// pp-exact:dollar-crate.pp\n10\t\n-\tfn main() { { ::std::io::_print(format_args!(\"rust\\n\")); }; }\n+\tfn main() {\n+\t    { ::std::io::_print(format_args!(\"{0}\\n\", format_args!(\"rust\"))); };\n+\t}\n12\t\n\n\n"
F.........................................................
failures:

---- [pretty] tests/pretty/dollar-crate.rs stdout ----


error: pretty-printed source does not match expected source
expected:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// pretty-compare-only
// pretty-mode:expanded
// pretty-mode:expanded
// pp-exact:dollar-crate.pp

fn main() { { ::std::io::_print(format_args!("rust\n")); }; }
------------------------------------------
actual:
------------------------------------------
#![feature(prelude_import)]
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
Build completed unsuccessfully in 0:13:43
extern crate std;
// pretty-compare-only
// pretty-mode:expanded
// pretty-mode:expanded
// pp-exact:dollar-crate.pp
fn main() {
fn main() {
    { ::std::io::_print(format_args!("{0}\n", format_args!("rust"))); };

------------------------------------------
diff:
------------------------------------------
------------------------------------------
8 // pretty-mode:expanded
9 // pp-exact:dollar-crate.pp
10 
- fn main() { { ::std::io::_print(format_args!("rust\n")); }; }
+ fn main() {
+     { ::std::io::_print(format_args!("{0}\n", format_args!("rust"))); };
12 



