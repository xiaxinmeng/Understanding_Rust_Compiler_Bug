plain
 finished in 44.513 seconds
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
..........................................2022-09-08T11:05:01.504436Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\nfn foo(_: [i32; (3 as usize)]) ({ } as ())\n\nfn bar() ({\n        const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n        let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n        let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n        let _ =\n            (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3]) as\n                        &[i32; 3]) as *const _ as *const [i32; 3]) as\n                *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n        ({\n                let res =\n                    ((::alloc::fmt::format as\n                            for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1\n                                as\n                                fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n                                            as &str)] as [&str; 1]) as &[&str; 1]),\n                            (&([] as [ArgumentV1; 0]) as &[ArgumentV1; 0])) as\n                            Arguments)) as String);\n                (res as String)\n            } as String);\n    } as ())\ntype Foo = [i32; (3 as usize)];\nstruct Bar {\n    x: [i32; (3 as usize)],\n}\nstruct TupleBar([i32; (4 as usize)]);\nenum Baz { BazVariant([i32; (5 as usize)]), }\nfn id<T>(x: T) -> T ({ (x as T) } as T)\nfn use_id() ({\n        let _ =\n            ((id::<[i32; (3 as usize)]> as\n                    fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1 as i32),\n                        (2 as i32), (3 as i32)] as [i32; 3])) as [i32; 3]);\n    } as ())\nfn main() ({ } as ())\n\n------------------------------------------\nactual:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\nfn foo(_: [i32; (3 as usize)]) ({ } as ())\n\nfn bar() ({\n        const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n        let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n        let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n        let _ =\n            (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3]) as\n                        &[i32; 3]) as *const _ as *const [i32; 3]) as\n                *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n        ({\n                let res =\n                    ((::alloc::fmt::format as\n                            for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1\n                                as\n                                fn(&[&'static str], &[core::fmt::ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n                                            as &str)] as [&str; 1]) as &[&str; 1]),\n                            (&([] as [core::fmt::ArgumentV1; 0]) as\n                                &[core::fmt::ArgumentV1; 0])) as Arguments)) as String);\n                (res as String)\n            } as String);\n    } as ())\ntype Foo = [i32; (3 as usize)];\nstruct Bar {\n    x: [i32; (3 as usize)],\n}\nstruct TupleBar([i32; (4 as usize)]);\nenum Baz { BazVariant([i32; (5 as usize)]), }\nfn id<T>(x: T) -> T ({ (x as T) } as T)\nfn use_id() ({\n        let _ =\n            ((id::<[i32; (3 as usize)]> as\n                    fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1 as i32),\n                        (2 as i32), (3 as i32)] as [i32; 3])) as [i32; 3]);\n    } as ())\nfn main() ({ } as ())\n\n------------------------------------------\ndiff:\n------------------------------------------\n34\t                    ((::alloc::fmt::format as\n35\t                            for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1\n36\t                                as\n-\t                                fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n+\t                                fn(&[&'static str], &[core::fmt::ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n38\t                                            as &str)] as [&str; 1]) as &[&str; 1]),\n-\t                            (&([] as [ArgumentV1; 0]) as &[ArgumentV1; 0])) as\n-\t                            Arguments)) as String);\n+\t                            (&([] as [core::fmt::ArgumentV1; 0]) as\n+\t                                &[core::fmt::ArgumentV1; 0])) as Arguments)) as String);\n41\t                (res as String)\n42\t            } as String);\n43\t    } as ())\n\n\n"
Some tests failed in compiletest suite=pretty mode=pretty host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F............................

---- [pretty] src/test/pretty/issue-4264.rs stdout ----

error: pretty-printed source does not match expected source
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
                                fn(&[&'static str], &[core::fmt::ArgumentV1]) -> Arguments {Arguments::new_v1})((&([("test"
                                            as &str)] as [&str; 1]) as &[&str; 1]),
                            (&([] as [core::fmt::ArgumentV1; 0]) as
                                &[core::fmt::ArgumentV1; 0])) as Arguments)) as String);
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
------------------------------------------
34                     ((::alloc::fmt::format as
35                             for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1
36                                 as
-                                 fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([("test"
+                                 fn(&[&'static str], &[core::fmt::ArgumentV1]) -> Arguments {Arguments::new_v1})((&([("test"
38                                             as &str)] as [&str; 1]) as &[&str; 1]),
-                             (&([] as [ArgumentV1; 0]) as &[ArgumentV1; 0])) as
-                             Arguments)) as String);
+                             (&([] as [core::fmt::ArgumentV1; 0]) as
+                                 &[core::fmt::ArgumentV1; 0])) as Arguments)) as String);
41                 (res as String)
42             } as String);
43     } as ())


thread '[pretty] src/test/pretty/issue-4264.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2229:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
