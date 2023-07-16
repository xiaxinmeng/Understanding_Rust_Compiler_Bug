plain
.................................................................................................... 9200/11455
.................................................................................................... 9300/11455
.................................................................................................... 9400/11455
..............i......i.............................................................................. 9500/11455
....................................................iiiiiii..iiiiii.i............................... 9600/11455
.................................................................................................... 9800/11455
.................................................................................................... 9900/11455
.................................................................................................... 10000/11455
.................................................................................................... 10100/11455
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.075 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i...i.i...ii...i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.23s

 finished in 2.297 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/pretty") not skipped for "bootstrap::test::Pretty" -- not in ["src/tools/tidy"]
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 69 tests
2021-02-17T15:12:07.538309Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#![feature(asm)]\n#[prelude_import]\nuse ::std::prelude::v1::*;\n#[macro_use]\nextern crate std;\n\n// pretty-mode:expanded\n// pp-exact:asm.pp\n// only-x86_64\n\npub fn main() {\n    let a: i32;\n    let mut b = 4i32;\n    unsafe {\n        asm!(\"\");\n        asm!(\"\");\n        asm!(\"\", options(nomem, nostack));\n        asm!(\"{0}\", in(reg) 4);\n        asm!(\"{0}\", out(reg) a);\n        asm!(\"{0}\", inout(reg) b);\n        asm!(\"{0} {1}\", out(reg) _, inlateout(reg) b => _);\n        asm!(\"\", out(\"al\") _, lateout(\"rbx\") _);\n        asm!(\"inst1\\ninst2\");\n        asm!(\"inst1 {0}, 42\\ninst2 {1}, 24\", in(reg) a, out(reg) b);\n        asm!(\"inst2 {1}, 24\\ninst1 {0}, 42\", in(reg) a, out(reg) b);\n        asm!(\"inst1 {0}, 42\\ninst2 {1}, 24\", in(reg) a, out(reg) b);\n        asm!(\"inst1\\ninst2\");\n        asm!(\"inst1\\ninst2\");\n        asm!(\"inst1\\n\\tinst2\");\n        asm!(\"inst1\\ninst2\\ninst3\\ninst4\");\n    }\n}\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#![feature(asm)]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n\n// pretty-mode:expanded\n// pp-exact:asm.pp\n// only-x86_64\n\npub fn main() {\n    let a: i32;\n    let mut b = 4i32;\n    unsafe {\n        asm!(\"\");\n        asm!(\"\");\n        asm!(\"\", options(nomem, nostack));\n        asm!(\"{0}\", in(reg) 4);\n        asm!(\"{0}\", out(reg) a);\n        asm!(\"{0}\", inout(reg) b);\n        asm!(\"{0} {1}\", out(reg) _, inlateout(reg) b => _);\n        asm!(\"\", out(\"al\") _, lateout(\"rbx\") _);\n        asm!(\"inst1\\ninst2\");\n        asm!(\"inst1 {0}, 42\\ninst2 {1}, 24\", in(reg) a, out(reg) b);\n        asm!(\"inst2 {1}, 24\\ninst1 {0}, 42\", in(reg) a, out(reg) b);\n        asm!(\"inst1 {0}, 42\\ninst2 {1}, 24\", in(reg) a, out(reg) b);\n        asm!(\"inst1\\ninst2\");\n        asm!(\"inst1\\ninst2\");\n        asm!(\"inst1\\n\\tinst2\");\n        asm!(\"inst1\\ninst2\\ninst3\\ninst4\");\n    }\n}\n\n------------------------------------------\ndiff:\n------------------------------------------\n2\t#![no_std]\n3\t#![feature(asm)]\n4\t#[prelude_import]\n-\tuse ::std::prelude::v1::*;\n+\tuse ::std::prelude::rust_2015::*;\n6\t#[macro_use]\n7\textern crate std;\n8\t\n\n\n"
F2021-02-17T15:12:07.548976Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::v1::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:cast-lt.pp\n\nmacro_rules! negative { ($ e : expr) => { $ e < 0 } }\n\nfn main() { (1 as i32) < 0; }\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:cast-lt.pp\n\nmacro_rules! negative { ($ e : expr) => { $ e < 0 } }\n\nfn main() { (1 as i32) < 0; }\n\n------------------------------------------\ndiff:\n------------------------------------------\n1\t#![feature(prelude_import)]\n2\t#![no_std]\n3\t#[prelude_import]\n-\tuse ::std::prelude::v1::*;\n+\tuse ::std::prelude::rust_2015::*;\n5\t#[macro_use]\n6\textern crate std;\n7\t// pretty-compare-only\n\n\n"
F............2021-02-17T15:12:07.612584Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::v1::*;\n#[macro_use]\nextern crate std;\n// Test for issue 80832\n//\n// pretty-mode:expanded\n// pp-exact:expanded-and-path-remap-80832.pp\n// compile-flags: --remap-path-prefix {{src-base}}=the/src\n\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// Test for issue 80832\n//\n// pretty-mode:expanded\n// pp-exact:expanded-and-path-remap-80832.pp\n// compile-flags: --remap-path-prefix {{src-base}}=the/src\n\nfn main() { }\n\n------------------------------------------\ndiff:\n------------------------------------------\n1\t#![feature(prelude_import)]\n2\t#![no_std]\n3\t#[prelude_import]\n-\tuse ::std::prelude::v1::*;\n+\tuse ::std::prelude::rust_2015::*;\n5\t#[macro_use]\n6\textern crate std;\n7\t// Test for issue 80832\n\n\n"
F2021-02-17T15:12:07.613580Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::v1::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:dollar-crate.pp\n\nfn main() {\n    {\n        ::std::io::_print(::core::fmt::Arguments::new_v1(&[\"rust\\n\"],\n                                                         &match () {\n                                                              () => [],\n                                                          }));\n    };\n}\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:dollar-crate.pp\n\nfn main() {\n    {\n        ::std::io::_print(::core::fmt::Arguments::new_v1(&[\"rust\\n\"],\n                                                         &match () {\n                                                              () => [],\n                                                          }));\n    };\n}\n\n------------------------------------------\ndiff:\n------------------------------------------\n1\t#![feature(prelude_import)]\n2\t#![no_std]\n3\t#[prelude_import]\n-\tuse ::std::prelude::v1::*;\n+\tuse ::std::prelude::rust_2015::*;\n5\t#[macro_use]\n6\textern crate std;\n7\t// pretty-compare-only\n\n\n"
F...........2021-02-17T15:12:07.674704Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::v1::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:issue-12590-c.pp\n\n// The next line should be expanded\n\n#[path = \"issue-12590-b.rs\"]\nmod issue_12590_b {\n\n    fn b() { }\n    fn main() { }\n}\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:issue-12590-c.pp\n\n// The next line should be expanded\n\n#[path = \"issue-12590-b.rs\"]\nmod issue_12590_b {\n\n    fn b() { }\n    fn main() { }\n}\nfn main() { }\n\n------------------------------------------\ndiff:\n------------------------------------------\n1\t#![feature(prelude_import)]\n2\t#![no_std]\n3\t#[prelude_import]\n-\tuse ::std::prelude::v1::*;\n+\tuse ::std::prelude::rust_2015::*;\n5\t#[macro_use]\n6\textern crate std;\n7\t// pretty-compare-only\n\n\n"
.F.......2021-02-17T15:12:07.740524Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::v1::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\npub fn foo(_: [i32; (3 as usize)]) ({ } as ())\n\npub fn bar() ({\n                  const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n                  let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _ =\n                      (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])\n                            as &[i32; 3]) as *const _ as *const [i32; 3]) as\n                          *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n                  ({\n                       let res =\n                           ((::alloc::fmt::format as\n                                for<\'r> fn(Arguments<\'r>) -> String {format})(((::core::fmt::Arguments::new_v1\n                                                                                   as\n                                                                                   fn(&[&\'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n                                                                                                                                                                as\n                                                                                                                                                                &str)]\n                                                                                                                                                              as\n                                                                                                                                                              [&str; 1])\n                                                                                                                                                            as\n                                                                                                                                                            &[&str; 1]),\n                                                                                                                                                        (&(match (()\n                                                                                                                                                                     as\n                                                                                                                                                                     ())\n                                                                                                                                                               {\n                                                                                                                                                               ()\n                                                                                                                                                               =>\n                                                                                                                                                               ([]\n                                                                                                                                                                   as\n                                                                                                                                                                   [ArgumentV1; 0]),\n                                                                                                                                                           }\n                                                                                                                                                              as\n                                                                                                                                                              [ArgumentV1; 0])\n                                                                                                                                                            as\n                                                                                                                                                            &[ArgumentV1; 0]))\n                                                                                  as\n                                                                                  Arguments))\n                               as String);\n                       (res as String)\n                   } as String);\n              } as ())\npub type Foo = [i32; (3 as usize)];\npub struct Bar {\n    pub x: [i32; (3 as usize)],\n}\npub struct TupleBar([i32; (4 as usize)]);\npub enum Baz { BazVariant([i32; (5 as usize)]), }\npub fn id<T>(x: T) -> T ({ (x as T) } as T)\npub fn use_id() ({\n                     let _ =\n                         ((id::<[i32; (3 as usize)]> as\n                              fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1\n                                                                               as\n                                                                               i32),\n                                                                           (2\n                                                                               as\n                                                                               i32),\n                                                                           (3\n                                                                               as\n                                                                               i32)]\n                                                                             as\n                                                                             [i32; 3]))\n                             as [i32; 3]);\n                 } as ())\nfn main() ({ } as ())\n\n------------------------------------------\nactual:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\npub fn foo(_: [i32; (3 as usize)]) ({ } as ())\n\npub fn bar() ({\n                  const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n                  let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _ =\n                      (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])\n                            as &[i32; 3]) as *const _ as *const [i32; 3]) as\n                          *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n                  ({\n                       let res =\n                           ((::alloc::fmt::format as\n                                for<\'r> fn(Arguments<\'r>) -> String {format})(((::core::fmt::Arguments::new_v1\n                                                                                   as\n                                                                                   fn(&[&\'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n                                                                                                                                                                as\n                                                                                                                                                                &str)]\n                                                                                                                                                              as\n                                                                                                                                                              [&str; 1])\n                                                                                                                                                            as\n                                                                                                                                                            &[&str; 1]),\n                                                                                                                                                        (&(match (()\n                                                                                                                                                                     as\n                                                                                                                                                                     ())\n                                                                                                                                                               {\n                                                                                                                                                               ()\n                                                                                                                                                               =>\n                                                                                                                                                               ([]\n                                                                                                                                                                   as\n                                                                                                                                                                   [ArgumentV1; 0]),\n                                                                                                                                                           }\n                                                                                                                                                              as\n                                                                                                                                                              [ArgumentV1; 0])\n                                                                                                                                                            as\n                                                                                                                                                            &[ArgumentV1; 0]))\n                                                                                  as\n                                                                                  Arguments))\n                               as String);\n                       (res as String)\n                   } as String);\n              } as ())\npub type Foo = [i32; (3 as usize)];\npub struct Bar {\n    pub x: [i32; (3 as usize)],\n}\npub struct TupleBar([i32; (4 as usize)]);\npub enum Baz { BazVariant([i32; (5 as usize)]), }\npub fn id<T>(x: T) -> T ({ (x as T) } as T)\npub fn use_id() ({\n                     let _ =\n                         ((id::<[i32; (3 as usize)]> as\n                              fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1\n                                                                               as\n                                                                               i32),\n                                                                           (2\n                                                                               as\n                                                                               i32),\n                                                                           (3\n                                                                               as\n                                                                               i32)]\n                                                                             as\n                                                                             [i32; 3]))\n                             as [i32; 3]);\n                 } as ())\nfn main() ({ } as ())\n\n------------------------------------------\ndiff:\n------------------------------------------\n1\t#[prelude_import]\n-\tuse ::std::prelude::v1::*;\n+\tuse ::std::prelude::rust_2015::*;\n3\t#[macro_use]\n4\textern crate std;\n5\t// pretty-compare-only\n\n\n"
Some tests failed in compiletest suite=pretty mode=pretty host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F................................

---- [pretty] pretty/asm.rs stdout ----

error: pretty-printed source does not match expected source
error: pretty-printed source does not match expected source
expected:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#![feature(asm)]
#[prelude_import]
use ::std::prelude::v1::*;
extern crate std;


// pretty-mode:expanded
// pp-exact:asm.pp
// only-x86_64
pub fn main() {
    let a: i32;
    let a: i32;
    let mut b = 4i32;
    unsafe {
        asm!("");
        asm!("");
        asm!("", options(nomem, nostack));
        asm!("{0}", in(reg) 4);
        asm!("{0}", out(reg) a);
        asm!("{0}", inout(reg) b);
        asm!("{0} {1}", out(reg) _, inlateout(reg) b => _);
        asm!("", out("al") _, lateout("rbx") _);
        asm!("inst1\ninst2");
        asm!("inst1 {0}, 42\ninst2 {1}, 24", in(reg) a, out(reg) b);
        asm!("inst2 {1}, 24\ninst1 {0}, 42", in(reg) a, out(reg) b);
        asm!("inst1 {0}, 42\ninst2 {1}, 24", in(reg) a, out(reg) b);
        asm!("inst1\ninst2");
        asm!("inst1\ninst2");
        asm!("inst1\n\tinst2");
        asm!("inst1\ninst2\ninst3\ninst4");
}

------------------------------------------
actual:
actual:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#![feature(asm)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;


// pretty-mode:expanded
// pp-exact:asm.pp
// only-x86_64
pub fn main() {
    let a: i32;
    let a: i32;
    let mut b = 4i32;
    unsafe {
        asm!("");
        asm!("");
        asm!("", options(nomem, nostack));
        asm!("{0}", in(reg) 4);
        asm!("{0}", out(reg) a);
        asm!("{0}", inout(reg) b);
        asm!("{0} {1}", out(reg) _, inlateout(reg) b => _);
        asm!("", out("al") _, lateout("rbx") _);
        asm!("inst1\ninst2");
        asm!("inst1 {0}, 42\ninst2 {1}, 24", in(reg) a, out(reg) b);
        asm!("inst2 {1}, 24\ninst1 {0}, 42", in(reg) a, out(reg) b);
        asm!("inst1 {0}, 42\ninst2 {1}, 24", in(reg) a, out(reg) b);
        asm!("inst1\ninst2");
        asm!("inst1\ninst2");
        asm!("inst1\n\tinst2");
        asm!("inst1\ninst2\ninst3\ninst4");
}

------------------------------------------
diff:
diff:
------------------------------------------
2 #![no_std]
3 #![feature(asm)]
4 #[prelude_import]
- use ::std::prelude::v1::*;
+ use ::std::prelude::rust_2015::*;
7 extern crate std;
8 




thread '[pretty] pretty/asm.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9

---- [pretty] pretty/cast-lt.rs stdout ----

error: pretty-printed source does not match expected source
error: pretty-printed source does not match expected source
expected:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
extern crate std;
// pretty-compare-only
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:cast-lt.pp

macro_rules! negative { ($ e : expr) => { $ e < 0 } }

fn main() { (1 as i32) < 0; }
------------------------------------------
actual:
------------------------------------------
#![feature(prelude_import)]
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// pretty-compare-only
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:cast-lt.pp

macro_rules! negative { ($ e : expr) => { $ e < 0 } }

fn main() { (1 as i32) < 0; }
------------------------------------------
diff:
------------------------------------------
1 #![feature(prelude_import)]
1 #![feature(prelude_import)]
2 #![no_std]
3 #[prelude_import]
- use ::std::prelude::v1::*;
+ use ::std::prelude::rust_2015::*;
6 extern crate std;
7 // pretty-compare-only




thread '[pretty] pretty/cast-lt.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9
---- [pretty] pretty/expanded-and-path-remap-80832.rs stdout ----

error: pretty-printed source does not match expected source
expected:
expected:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
extern crate std;
// Test for issue 80832
//
//
// pretty-mode:expanded
// pp-exact:expanded-and-path-remap-80832.pp
// compile-flags: --remap-path-prefix {{src-base}}=the/src
fn main() { }

------------------------------------------
actual:
actual:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// Test for issue 80832
//
//
// pretty-mode:expanded
// pp-exact:expanded-and-path-remap-80832.pp
// compile-flags: --remap-path-prefix {{src-base}}=the/src
fn main() { }

------------------------------------------
diff:
diff:
------------------------------------------
1 #![feature(prelude_import)]
2 #![no_std]
3 #[prelude_import]
- use ::std::prelude::v1::*;
+ use ::std::prelude::rust_2015::*;
6 extern crate std;
7 // Test for issue 80832




thread '[pretty] pretty/expanded-and-path-remap-80832.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9
---- [pretty] pretty/dollar-crate.rs stdout ----

error: pretty-printed source does not match expected source
expected:
expected:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
extern crate std;
// pretty-compare-only
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:dollar-crate.pp
fn main() {
    {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["rust\n"],
                                                         &match () {
                                                              () => [],
                                                          }));
}

------------------------------------------
actual:
actual:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// pretty-compare-only
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:dollar-crate.pp
fn main() {
    {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["rust\n"],
                                                         &match () {
                                                              () => [],
                                                          }));
}

------------------------------------------
diff:
diff:
------------------------------------------
1 #![feature(prelude_import)]
2 #![no_std]
3 #[prelude_import]
- use ::std::prelude::v1::*;
+ use ::std::prelude::rust_2015::*;
6 extern crate std;
7 // pretty-compare-only




thread '[pretty] pretty/dollar-crate.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9
---- [pretty] pretty/issue-12590-c.rs stdout ----

error: pretty-printed source does not match expected source
expected:
expected:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
extern crate std;
// pretty-compare-only
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:issue-12590-c.pp
// The next line should be expanded


#[path = "issue-12590-b.rs"]
mod issue_12590_b {

    fn b() { }
    fn main() { }
fn main() { }

------------------------------------------
actual:
actual:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// pretty-compare-only
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:issue-12590-c.pp
// The next line should be expanded


#[path = "issue-12590-b.rs"]
mod issue_12590_b {

    fn b() { }
    fn main() { }
fn main() { }

------------------------------------------
diff:
diff:
------------------------------------------
1 #![feature(prelude_import)]
2 #![no_std]
3 #[prelude_import]
- use ::std::prelude::v1::*;
+ use ::std::prelude::rust_2015::*;
6 extern crate std;
7 // pretty-compare-only




thread '[pretty] pretty/issue-12590-c.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9
---- [pretty] pretty/issue-4264.rs stdout ----

error: pretty-printed source does not match expected source
expected:
expected:
------------------------------------------
#[prelude_import]
use ::std::prelude::v1::*;
extern crate std;
// pretty-compare-only
// pretty-compare-only
// pretty-mode:hir,typed
// pp-exact:issue-4264.pp

// #4264 fixed-length vector types

pub fn foo(_: [i32; (3 as usize)]) ({ } as ())
pub fn bar() ({
pub fn bar() ({
                  const FOO: usize = ((5 as usize) - (4 as usize) as usize);
                  let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);

                  let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);
                  let _ =
                  let _ =
                      (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])
                            as &[i32; 3]) as *const _ as *const [i32; 3]) as
                          *const [i32; (3 as usize)] as *const [i32; 3]);









                  ({
                       let res =
                           ((::alloc::fmt::format as
                                for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1
                                                                                   as
                                                                                   fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([("test"
                                                                                                                                                                &str)]
                                                                                                                                                              as
                                                                                                                                                              as
                                                                                                                                                              [&str; 1])
                                                                                                                                                            as
                                                                                                                                                            &[&str; 1]),
                                                                                                                                                        (&(match (()
                                                                                                                                                                     ())
                                                                                                                                                               {
                                                                                                                                                               ()
                                                                                                                                                               =>
                                                                                                                                                               =>
                                                                                                                                                               ([]
                                                                                                                                                                   as
                                                                                                                                                                   [ArgumentV1; 0]),
                                                                                                                                                              as
                                                                                                                                                              as
                                                                                                                                                              [ArgumentV1; 0])
                                                                                                                                                            as
                                                                                                                                                            &[ArgumentV1; 0]))
                                                                                  Arguments))
                               as String);
                       (res as String)
                   } as String);
                   } as String);
              } as ())
pub type Foo = [i32; (3 as usize)];
pub struct Bar {
    pub x: [i32; (3 as usize)],
}
pub struct TupleBar([i32; (4 as usize)]);
pub enum Baz { BazVariant([i32; (5 as usize)]), }
pub fn id<T>(x: T) -> T ({ (x as T) } as T)
pub fn use_id() ({
                     let _ =
                         ((id::<[i32; (3 as usize)]> as
                              fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1
                                                                               i32),
                                                                           (2
                                                                               as
                                                                               i32),
                                                                               i32),
                                                                           (3
                                                                               as
                                                                               i32)]
                                                                             as
                                                                             [i32; 3]))
                             as [i32; 3]);
                 } as ())
fn main() ({ } as ())
------------------------------------------
actual:
------------------------------------------
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// pretty-compare-only
// pretty-compare-only
// pretty-mode:hir,typed
// pp-exact:issue-4264.pp

// #4264 fixed-length vector types

pub fn foo(_: [i32; (3 as usize)]) ({ } as ())
pub fn bar() ({
pub fn bar() ({
                  const FOO: usize = ((5 as usize) - (4 as usize) as usize);
                  let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);

                  let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);
                  let _ =
                  let _ =
                      (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])
                            as &[i32; 3]) as *const _ as *const [i32; 3]) as
                          *const [i32; (3 as usize)] as *const [i32; 3]);









                  ({
                       let res =
                           ((::alloc::fmt::format as
                                for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1
                                                                                   as
                                                                                   fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([("test"
                                                                                                                                                                &str)]
                                                                                                                                                              as
                                                                                                                                                              as
                                                                                                                                                              [&str; 1])
                                                                                                                                                            as
                                                                                                                                                            &[&str; 1]),
                                                                                                                                                        (&(match (()
                                                                                                                                                                     ())
                                                                                                                                                               {
                                                                                                                                                               ()
                                                                                                                                                               =>
                                                                                                                                                               =>
                                                                                                                                                               ([]
                                                                                                                                                                   as
                                                                                                                                                                   [ArgumentV1; 0]),
                                                                                                                                                              as
                                                                                                                                                              as
                                                                                                                                                              [ArgumentV1; 0])
                                                                                                                                                            as
                                                                                                                                                            &[ArgumentV1; 0]))
                                                                                  Arguments))
                               as String);
                       (res as String)
                   } as String);
                   } as String);
              } as ())
pub type Foo = [i32; (3 as usize)];
pub struct Bar {
---
test result: FAILED. 63 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.62s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "pretty" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:30
