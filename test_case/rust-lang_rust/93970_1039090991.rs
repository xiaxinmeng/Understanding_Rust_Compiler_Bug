plain
Suite(test::src/test/pretty) not skipped for "bootstrap::test::Pretty" -- not in [src/tools/tidy]
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 70 tests
..........................2022-02-14T13:28:04.271255Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir\n// pp-exact:hir-pretty-loop.pp\n\npub fn foo() { loop { break; } }\n\n------------------------------------------\nactual:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir\n// pp-exact:hir-pretty-loop.pp\n\nfn foo() { loop { break; } }\n\n------------------------------------------\ndiff:\n------------------------------------------\n6\t// pretty-mode:hir\n7\t// pp-exact:hir-pretty-loop.pp\n8\t\n-\tpub fn foo() { loop { break; } }\n+\tfn foo() { loop { break; } }\n10\t\n\n\n"
F..........2022-02-14T13:28:04.339273Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\npub fn foo(_: [i32; (3 as usize)]) ({ } as ())\n\npub fn bar() ({\n        const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n        let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n        let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n        let _ =\n            (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3]) as\n                        &[i32; 3]) as *const _ as *const [i32; 3]) as\n                *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n        ({\n                let res =\n                    ((::alloc::fmt::format as\n                            for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1\n                                as\n                                fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n                                            as &str)] as [&str; 1]) as &[&str; 1]),\n                            (&([] as [ArgumentV1; 0]) as &[ArgumentV1; 0])) as\n                            Arguments)) as String);\n                (res as String)\n            } as String);\n    } as ())\npub type Foo = [i32; (3 as usize)];\npub struct Bar {\n    pub x: [i32; (3 as usize)],\n}\npub struct TupleBar([i32; (4 as usize)]);\npub enum Baz { BazVariant([i32; (5 as usize)]), }\npub fn id<T>(x: T) -> T ({ (x as T) } as T)\npub fn use_id() ({\n        let _ =\n            ((id::<[i32; (3 as usize)]> as\n                    fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1 as i32),\n                        (2 as i32), (3 as i32)] as [i32; 3])) as [i32; 3]);\n    } as ())\nfn main() ({ } as ())\n\n------------------------------------------\nactual:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\nfn foo(_: [i32; (3 as usize)]) ({ } as ())\n\nfn bar() ({\n        const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n        let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n        let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n        let _ =\n            (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3]) as\n                        &[i32; 3]) as *const _ as *const [i32; 3]) as\n                *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n        ({\n                let res =\n                    ((::alloc::fmt::format as\n                            for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1\n                                as\n                                fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n                                            as &str)] as [&str; 1]) as &[&str; 1]),\n                            (&([] as [ArgumentV1; 0]) as &[ArgumentV1; 0])) as\n                            Arguments)) as String);\n                (res as String)\n            } as String);\n    } as ())\ntype Foo = [i32; (3 as usize)];\nstruct Bar {\n    x: [i32; (3 as usize)],\n}\nstruct TupleBar([i32; (4 as usize)]);\nenum Baz { BazVariant([i32; (5 as usize)]), }\nfn id<T>(x: T) -> T ({ (x as T) } as T)\nfn use_id() ({\n        let _ =\n            ((id::<[i32; (3 as usize)]> as\n                    fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1 as i32),\n                        (2 as i32), (3 as i32)] as [i32; 3])) as [i32; 3]);\n    } as ())\nfn main() ({ } as ())\n\n------------------------------------------\ndiff:\n------------------------------------------\n8\t\n9\t// #4264 fixed-length vector types\n10\t\n-\tpub fn foo(_: [i32; (3 as usize)]) ({ } as ())\n+\tfn foo(_: [i32; (3 as usize)]) ({ } as ())\n12\t\n-\tpub fn bar() ({\n+\tfn bar() ({\n14\t        const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n15\t        let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n16\t\n\n41\t                (res as String)\n42\t            } as String);\n43\t    } as ())\n-\tpub type Foo = [i32; (3 as usize)];\n-\tpub struct Bar {\n-\t    pub x: [i32; (3 as usize)],\n+\ttype Foo = [i32; (3 as usize)];\n+\tstruct Bar {\n+\t    x: [i32; (3 as usize)],\n47\t}\n-\tpub struct TupleBar([i32; (4 as usize)]);\n-\tpub enum Baz { BazVariant([i32; (5 as usize)]), }\n-\tpub fn id<T>(x: T) -> T ({ (x as T) } as T)\n-\tpub fn use_id() ({\n+\tstruct TupleBar([i32; (4 as usize)]);\n+\tenum Baz { BazVariant([i32; (5 as usize)]), }\n+\tfn id<T>(x: T) -> T ({ (x as T) } as T)\n+\tfn use_id() ({\n52\t        let _ =\n53\t            ((id::<[i32; (3 as usize)]> as\n54\t                    fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1 as i32),\n\n\n"
Some tests failed in compiletest suite=pretty mode=pretty host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..F..............................

---- [pretty] pretty/hir-pretty-loop.rs stdout ----

error: pretty-printed source does not match expected source
error: pretty-printed source does not match expected source
expected:
------------------------------------------
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// pretty-compare-only
// pretty-mode:hir
// pretty-mode:hir
// pp-exact:hir-pretty-loop.pp

pub fn foo() { loop { break; } }
------------------------------------------
actual:
------------------------------------------
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// pretty-compare-only
// pretty-mode:hir
// pretty-mode:hir
// pp-exact:hir-pretty-loop.pp

fn foo() { loop { break; } }
------------------------------------------
diff:
------------------------------------------
------------------------------------------
6 // pretty-mode:hir
7 // pp-exact:hir-pretty-loop.pp
8 
- pub fn foo() { loop { break; } }
+ fn foo() { loop { break; } }



thread '[pretty] pretty/hir-pretty-loop.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2098:9
thread '[pretty] pretty/hir-pretty-loop.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2098:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [pretty] pretty/issue-4264.rs stdout ----

error: pretty-printed source does not match expected source
expected:
------------------------------------------
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
            (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3]) as
                        &[i32; 3]) as *const _ as *const [i32; 3]) as
                *const [i32; (3 as usize)] as *const [i32; 3]);









        ({
                let res =
                    ((::alloc::fmt::format as
                            for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1
                                as
                                fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([("test"
                                            as &str)] as [&str; 1]) as &[&str; 1]),
                            (&([] as [ArgumentV1; 0]) as &[ArgumentV1; 0])) as
                            Arguments)) as String);
                (res as String)
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
                    fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1 as i32),
                        (2 as i32), (3 as i32)] as [i32; 3])) as [i32; 3]);
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

fn foo(_: [i32; (3 as usize)]) ({ } as ())
fn bar() ({
fn bar() ({
        const FOO: usize = ((5 as usize) - (4 as usize) as usize);
        let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);

        let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);
        let _ =
        let _ =
            (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3]) as
                        &[i32; 3]) as *const _ as *const [i32; 3]) as
                *const [i32; (3 as usize)] as *const [i32; 3]);









        ({
                let res =
                    ((::alloc::fmt::format as
                            for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1
                                as
                                fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([("test"
                                            as &str)] as [&str; 1]) as &[&str; 1]),
                            (&([] as [ArgumentV1; 0]) as &[ArgumentV1; 0])) as
                            Arguments)) as String);
                (res as String)
            } as String);
    } as ())
type Foo = [i32; (3 as usize)];
struct Bar {
    x: [i32; (3 as usize)],
}
struct TupleBar([i32; (4 as usize)]);
enum Baz { BazVariant([i32; (5 as usize)]), }
fn id<T>(x: T) -> T ({ (x as T) } as T)
fn use_id() ({
        let _ =
            ((id::<[i32; (3 as usize)]> as
                    fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1 as i32),
                        (2 as i32), (3 as i32)] as [i32; 3])) as [i32; 3]);
    } as ())
fn main() ({ } as ())
------------------------------------------
diff:
------------------------------------------
8 
8 
9 // #4264 fixed-length vector types
10 
- pub fn foo(_: [i32; (3 as usize)]) ({ } as ())
+ fn foo(_: [i32; (3 as usize)]) ({ } as ())
- pub fn bar() ({
+ fn bar() ({
+ fn bar() ({
14         const FOO: usize = ((5 as usize) - (4 as usize) as usize);
15         let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);

41                 (res as String)
42             } as String);
43     } as ())
43     } as ())
- pub type Foo = [i32; (3 as usize)];
- pub struct Bar {
-     pub x: [i32; (3 as usize)],
+ type Foo = [i32; (3 as usize)];
+ struct Bar {
+     x: [i32; (3 as usize)],
47 }
- pub struct TupleBar([i32; (4 as usize)]);
- pub enum Baz { BazVariant([i32; (5 as usize)]), }
- pub fn id<T>(x: T) -> T ({ (x as T) } as T)
- pub fn use_id() ({
+ struct TupleBar([i32; (4 as usize)]);
+ enum Baz { BazVariant([i32; (5 as usize)]), }
+ fn id<T>(x: T) -> T ({ (x as T) } as T)
+ fn use_id() ({
52         let _ =
53             ((id::<[i32; (3 as usize)]> as
54                     fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1 as i32),


thread '[pretty] pretty/issue-4264.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2098:9

