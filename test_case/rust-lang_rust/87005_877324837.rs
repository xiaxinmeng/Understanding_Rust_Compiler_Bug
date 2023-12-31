plain
Suite("src/test/pretty") not skipped for "bootstrap::test::Pretty" -- not in ["src/tools/tidy"]
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
...............2021-07-09T16:57:21.201198Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:dollar-crate.pp\n\nfn main() {\n    {\n        ::std::io::_print(::core::fmt::Arguments::new_v1(&[\"rust\\n\"],\n                                                         &match () {\n                                                              () => [],\n                                                          }));\n    };\n}\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:dollar-crate.pp\n\nfn main() {\n    { ::std::io::_print(::core::fmt::Arguments::new_literal(&[\"rust\\n\"])); };\n}\n\n------------------------------------------\ndiff:\n------------------------------------------\n9\t// pp-exact:dollar-crate.pp\n10\t\n11\tfn main() {\n-\t    {\n-\t        ::std::io::_print(::core::fmt::Arguments::new_v1(&[\"rust\\n\"],\n-\t                                                         &match () {\n-\t                                                              () => [],\n-\t                                                          }));\n-\t    };\n+\t    { ::std::io::_print(::core::fmt::Arguments::new_literal(&[\"rust\\n\"])); };\n18\t}\n19\t\n\n\n"
F..........................2021-07-09T16:57:21.333447Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\npub fn foo(_: [i32; (3 as usize)]) ({ } as ())\n\npub fn bar() ({\n                  const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n                  let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _ =\n                      (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])\n                            as &[i32; 3]) as *const _ as *const [i32; 3]) as\n                          *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n                  ({\n                       let res =\n                           ((::alloc::fmt::format as\n                                for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1\n                                                                                   as\n                                                                                   fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n                                                                                                                                                                as\n                                                                                                                                                                &str)]\n                                                                                                                                                              as\n                                                                                                                                                              [&str; 1])\n                                                                                                                                                            as\n                                                                                                                                                            &[&str; 1]),\n                                                                                                                                                        (&(match (()\n                                                                                                                                                                     as\n                                                                                                                                                                     ())\n                                                                                                                                                               {\n                                                                                                                                                               ()\n                                                                                                                                                               =>\n                                                                                                                                                               ([]\n                                                                                                                                                                   as\n                                                                                                                                                                   [ArgumentV1; 0]),\n                                                                                                                                                           }\n                                                                                                                                                              as\n                                                                                                                                                              [ArgumentV1; 0])\n                                                                                                                                                            as\n                                                                                                                                                            &[ArgumentV1; 0]))\n                                                                                  as\n                                                                                  Arguments))\n                               as String);\n                       (res as String)\n                   } as String);\n              } as ())\npub type Foo = [i32; (3 as usize)];\npub struct Bar {\n    pub x: [i32; (3 as usize)],\n}\npub struct TupleBar([i32; (4 as usize)]);\npub enum Baz { BazVariant([i32; (5 as usize)]), }\npub fn id<T>(x: T) -> T ({ (x as T) } as T)\npub fn use_id() ({\n                     let _ =\n                         ((id::<[i32; (3 as usize)]> as\n                              fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1\n                                                                               as\n                                                                               i32),\n                                                                           (2\n                                                                               as\n                                                                               i32),\n                                                                           (3\n                                                                               as\n                                                                               i32)]\n                                                                             as\n                                                                             [i32; 3]))\n                             as [i32; 3]);\n                 } as ())\nfn main() ({ } as ())\n\n------------------------------------------\nactual:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\npub fn foo(_: [i32; (3 as usize)]) ({ } as ())\n\npub fn bar() ({\n                  const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n                  let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _ =\n                      (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])\n                            as &[i32; 3]) as *const _ as *const [i32; 3]) as\n                          *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n                  ({\n                       let res =\n                           ((::alloc::fmt::format as\n                                for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_literal\n                                                                                   as\n                                                                                   fn(&[&'static str; 1]) -> Arguments {Arguments::new_literal})((&([(\"test\"\n                                                                                                                                                         as\n                                                                                                                                                         &str)]\n                                                                                                                                                       as\n                                                                                                                                                       [&str; 1])\n                                                                                                                                                     as\n                                                                                                                                                     &[&str; 1]))\n                                                                                  as\n                                                                                  Arguments))\n                               as String);\n                       (res as String)\n                   } as String);\n              } as ())\npub type Foo = [i32; (3 as usize)];\npub struct Bar {\n    pub x: [i32; (3 as usize)],\n}\npub struct TupleBar([i32; (4 as usize)]);\npub enum Baz { BazVariant([i32; (5 as usize)]), }\npub fn id<T>(x: T) -> T ({ (x as T) } as T)\npub fn use_id() ({\n                     let _ =\n                         ((id::<[i32; (3 as usize)]> as\n                              fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1\n                                                                               as\n                                                                               i32),\n                                                                           (2\n                                                                               as\n                                                                               i32),\n                                                                           (3\n                                                                               as\n                                                                               i32)]\n                                                                             as\n                                                                             [i32; 3]))\n                             as [i32; 3]);\n                 } as ())\nfn main() ({ } as ())\n\n------------------------------------------\ndiff:\n------------------------------------------\n32\t                  ({\n33\t                       let res =\n34\t                           ((::alloc::fmt::format as\n-\t                                for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1\n+\t                                for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_literal\n36\t                                                                                   as\n-\t                                                                                   fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([(\"test\"\n-\t                                                                                                                                                                as\n-\t                                                                                                                                                                &str)]\n-\t                                                                                                                                                              as\n-\t                                                                                                                                                              [&str; 1])\n-\t                                                                                                                                                            as\n-\t                                                                                                                                                            &[&str; 1]),\n-\t                                                                                                                                                        (&(match (()\n-\t                                                                                                                                                                     as\n-\t                                                                                                                                                                     ())\n-\t                                                                                                                                                               {\n-\t                                                                                                                                                               ()\n-\t                                                                                                                                                               =>\n-\t                                                                                                                                                               ([]\n-\t                                                                                                                                                                   as\n-\t                                                                                                                                                                   [ArgumentV1; 0]),\n-\t                                                                                                                                                           }\n-\t                                                                                                                                                              as\n-\t                                                                                                                                                              [ArgumentV1; 0])\n-\t                                                                                                                                                            as\n-\t                                                                                                                                                            &[ArgumentV1; 0]))\n+\t                                                                                   fn(&[&'static str; 1]) -> Arguments {Arguments::new_literal})((&([(\"test\"\n+\t                                                                                                                                                         as\n+\t                                                                                                                                                         &str)]\n+\t                                                                                                                                                       as\n+\t                                                                                                                                                       [&str; 1])\n+\t                                                                                                                                                     as\n+\t                                                                                                                                                     &[&str; 1]))\n58\t                                                                                  as\n59\t                                                                                  Arguments))\n60\t                               as String);\n\n\n"
Some tests failed in compiletest suite=pretty mode=pretty host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F............................

---- [pretty] pretty/dollar-crate.rs stdout ----

error: pretty-printed source does not match expected source
error: pretty-printed source does not match expected source
expected:
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
    };
}

------------------------------------------
------------------------------------------
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
fn main() {
    { ::std::io::_print(::core::fmt::Arguments::new_literal(&["rust\n"])); };

------------------------------------------
diff:
------------------------------------------
------------------------------------------
9 // pp-exact:dollar-crate.pp
11 fn main() {
-     {
-     {
-         ::std::io::_print(::core::fmt::Arguments::new_v1(&["rust\n"],
-                                                          &match () {
-                                                               () => [],
-     };
-     };
+     { ::std::io::_print(::core::fmt::Arguments::new_literal(&["rust\n"])); };
19 




thread '[pretty] pretty/dollar-crate.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2272:9

---- [pretty] pretty/issue-4264.rs stdout ----

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
                                for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_literal
                                                                                   as
                                                                                   fn(&[&'static str; 1]) -> Arguments {Arguments::new_literal})((&([("test"
                                                                                                                                                         &str)]
                                                                                                                                                       as
                                                                                                                                                       as
                                                                                                                                                       [&str; 1])
                                                                                                                                                     as
                                                                                                                                                     &[&str; 1]))
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
diff:
------------------------------------------
32                   ({
32                   ({
33                        let res =
34                            ((::alloc::fmt::format as
-                                 for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_v1
+                                 for<'r> fn(Arguments<'r>) -> String {format})(((::core::fmt::Arguments::new_literal
36                                                                                    as
-                                                                                    fn(&[&'static str], &[ArgumentV1]) -> Arguments {Arguments::new_v1})((&([("test"
-                                                                                                                                                                 &str)]
-                                                                                                                                                               as
-                                                                                                                                                               as
-                                                                                                                                                               [&str; 1])
-                                                                                                                                                             as
-                                                                                                                                                             &[&str; 1]),
-                                                                                                                                                         (&(match (()
-                                                                                                                                                                      ())
-                                                                                                                                                                {
-                                                                                                                                                                ()
-                                                                                                                                                                =>
-                                                                                                                                                                =>
-                                                                                                                                                                ([]
-                                                                                                                                                                    as
-                                                                                                                                                                    [ArgumentV1; 0]),
-                                                                                                                                                               as
-                                                                                                                                                               as
-                                                                                                                                                               [ArgumentV1; 0])
-                                                                                                                                                             as
-                                                                                                                                                             &[ArgumentV1; 0]))
+                                                                                    fn(&[&'static str; 1]) -> Arguments {Arguments::new_literal})((&([("test"
+                                                                                                                                                          &str)]
+                                                                                                                                                        as
+                                                                                                                                                        as
+                                                                                                                                                        [&str; 1])
+                                                                                                                                                      as
+                                                                                                                                                      &[&str; 1]))
59                                                                                   Arguments))
60                                as String);




thread '[pretty] pretty/issue-4264.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2272:9

failures:
    [pretty] pretty/dollar-crate.rs
    [pretty] pretty/issue-4264.rs
    [pretty] pretty/issue-4264.rs

test result: FAILED. 69 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "pretty" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:08
