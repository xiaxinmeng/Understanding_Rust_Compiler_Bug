plain
 finished in 31.355 seconds
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
2021-07-03T23:29:05.906445Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:cast-lt.pp\n\nmacro_rules! negative { ($ e : expr) => { $ e < 0 } }\n\nfn main() { (1 as i32) < 0; }\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:cast-lt.pp\n\nmacro_rules! negative { ($e : expr) => { $e < 0 } }\n\nfn main() { (1 as i32) < 0; }\n\n------------------------------------------\ndiff:\n------------------------------------------\n8\t// pretty-mode:expanded\n9\t// pp-exact:cast-lt.pp\n10\t\n-\tmacro_rules! negative { ($ e : expr) => { $ e < 0 } }\n+\tmacro_rules! negative { ($e : expr) => { $e < 0 } }\n12\t\n13\tfn main() { (1 as i32) < 0; }\n14\t\n\n\n"
F.2021-07-03T23:29:05.932317Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// pp-exact\n\n#![feature(rustc_attrs)]\n\nmacro_rules! mac { ($ ($ tt : tt) *) => () }\n\nmac! {\n    struct S { field1 : u8, field2 : u16, } impl Clone for S\n    {\n        fn clone() -> S\n        {\n            panic! () ;\n\n        }\n    }\n}\n\nmac! {\n    a(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n      aaaaaaaa aaaaaaaa) a\n    [aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa] a\n    {\n        aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n        aaaaaaaa aaaaaaaa aaaaaaaa\n    } a\n}\n\nmac!(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa);\nmac![aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa];\nmac! {\n    aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n    aaaaaaaa aaaaaaaa\n}\n\n#[rustc_dummy(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n              aaaaaaaa aaaaaaaa aaaaaaaa)]\n#[rustc_dummy[aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n              aaaaaaaa aaaaaaaa aaaaaaaa]]\n#[rustc_dummy {\n      aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n      aaaaaaaa aaaaaaaa\n  }]\n#[rustc_dummy =\n  \"aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\"]\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n// pp-exact\n\n#![feature(rustc_attrs)]\n\nmacro_rules! mac { ($($tt : tt) *) => () }\n\nmac! {\n    struct S { field1 : u8, field2 : u16, } impl Clone for S\n    {\n        fn clone() -> S\n        {\n            panic! () ;\n\n        }\n    }\n}\n\nmac! {\n    a(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n      aaaaaaaa aaaaaaaa) a\n    [aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa] a\n    {\n        aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n        aaaaaaaa aaaaaaaa aaaaaaaa\n    } a\n}\n\nmac!(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa);\nmac![aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa];\nmac! {\n    aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n    aaaaaaaa aaaaaaaa\n}\n\n#[rustc_dummy(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n              aaaaaaaa aaaaaaaa aaaaaaaa)]\n#[rustc_dummy[aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n              aaaaaaaa aaaaaaaa aaaaaaaa]]\n#[rustc_dummy {\n      aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n      aaaaaaaa aaaaaaaa\n  }]\n#[rustc_dummy =\n  \"aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\"]\nfn main() { }\n\n------------------------------------------\ndiff:\n------------------------------------------\n2\t\n3\t#![feature(rustc_attrs)]\n4\t\n-\tmacro_rules! mac { ($ ($ tt : tt) *) => () }\n+\tmacro_rules! mac { ($($tt : tt) *) => () }\n6\t\n7\tmac! {\n8\t    struct S { field1 : u8, field2 : u16, } impl Clone for S\n\n\n"
F..................................2021-07-03T23:29:06.092437Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// pp-exact\n\n#![feature(decl_macro)]\n\npub(crate) macro mac { ($ arg : expr) => { $ arg + $ arg } }\n\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n// pp-exact\n\n#![feature(decl_macro)]\n\npub(crate) macro mac { ($arg : expr) => { $arg + $arg } }\n\nfn main() { }\n\n------------------------------------------\ndiff:\n------------------------------------------\n2\t\n3\t#![feature(decl_macro)]\n4\t\n-\tpub(crate) macro mac { ($ arg : expr) => { $ arg + $ arg } }\n+\tpub(crate) macro mac { ($arg : expr) => { $arg + $arg } }\n6\t\n7\tfn main() { }\n8\t\n\n\n"
F2021-07-03T23:29:06.113891Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// pp-exact\n\nmacro_rules! brace { () => { } ; }\n\nmacro_rules! bracket[() => { } ;];\n\nmacro_rules! paren(() => { } ;);\n\nmacro_rules! matcher_brackets {\n    (paren) => { } ; (bracket) => { } ; (brace) => { } ;\n}\n\nmacro_rules! all_fragments {\n    ($ b : block, $ e : expr, $ i : ident, $ it : item, $ l : lifetime, $ lit\n     : literal, $ m : meta, $ p : pat, $ pth : path, $ s : stmt, $ tt : tt, $\n     ty : ty, $ vis : vis) => { } ;\n}\n\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n// pp-exact\n\nmacro_rules! brace { () => { } ; }\n\nmacro_rules! bracket[() => { } ;];\n\nmacro_rules! paren(() => { } ;);\n\nmacro_rules! matcher_brackets {\n    (paren) => { } ; (bracket) => { } ; (brace) => { } ;\n}\n\nmacro_rules! all_fragments {\n    ($b : block, $e : expr, $i : ident, $it : item, $l : lifetime, $lit :\n     literal, $m : meta, $p : pat, $pth : path, $s : stmt, $tt : tt, $ty : ty,\n     $vis : vis) => { } ;\n}\n\nfn main() { }\n\n------------------------------------------\ndiff:\n------------------------------------------\n11\t}\n12\t\n13\tmacro_rules! all_fragments {\n-\t    ($ b : block, $ e : expr, $ i : ident, $ it : item, $ l : lifetime, $ lit\n-\t     : literal, $ m : meta, $ p : pat, $ pth : path, $ s : stmt, $ tt : tt, $\n-\t     ty : ty, $ vis : vis) => { } ;\n+\t    ($b : block, $e : expr, $i : ident, $it : item, $l : lifetime, $lit :\n+\t     literal, $m : meta, $p : pat, $pth : path, $s : stmt, $tt : tt, $ty : ty,\n+\t     $vis : vis) => { } ;\n17\t}\n18\t\n19\tfn main() { }\n\n\n"
Some tests failed in compiletest suite=pretty mode=pretty host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F................................

---- [pretty] pretty/cast-lt.rs stdout ----

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

macro_rules! negative { ($e : expr) => { $e < 0 } }

fn main() { (1 as i32) < 0; }
------------------------------------------
diff:
------------------------------------------
------------------------------------------
8 // pretty-mode:expanded
9 // pp-exact:cast-lt.pp
10 
- macro_rules! negative { ($ e : expr) => { $ e < 0 } }
+ macro_rules! negative { ($e : expr) => { $e < 0 } }
12 
13 fn main() { (1 as i32) < 0; }




thread '[pretty] pretty/cast-lt.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2272:9

---- [pretty] pretty/delimited-token-groups.rs stdout ----

error: pretty-printed source does not match expected source
error: pretty-printed source does not match expected source
expected:
------------------------------------------
// pp-exact
#![feature(rustc_attrs)]


macro_rules! mac { ($ ($ tt : tt) *) => () }
mac! {
mac! {
    struct S { field1 : u8, field2 : u16, } impl Clone for S
        fn clone() -> S
        {
        {
            panic! () ;
        }
    }
}


mac! {
    a(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
      aaaaaaaa aaaaaaaa) a
    [aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa] a
    {
        aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
        aaaaaaaa aaaaaaaa aaaaaaaa
    } a


mac!(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa);
mac![aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa];
mac! {
    aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
    aaaaaaaa aaaaaaaa


#[rustc_dummy(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
              aaaaaaaa aaaaaaaa aaaaaaaa)]
#[rustc_dummy[aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
              aaaaaaaa aaaaaaaa aaaaaaaa]]
#[rustc_dummy {
      aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
      aaaaaaaa aaaaaaaa
#[rustc_dummy =
#[rustc_dummy =
  "aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa"]
fn main() { }
------------------------------------------
actual:
------------------------------------------
------------------------------------------
// pp-exact
#![feature(rustc_attrs)]


macro_rules! mac { ($($tt : tt) *) => () }
mac! {
mac! {
    struct S { field1 : u8, field2 : u16, } impl Clone for S
        fn clone() -> S
        {
        {
            panic! () ;
        }
    }
}


mac! {
    a(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
      aaaaaaaa aaaaaaaa) a
    [aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa] a
    {
        aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
        aaaaaaaa aaaaaaaa aaaaaaaa
    } a


mac!(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa);
mac![aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa];
mac! {
    aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
    aaaaaaaa aaaaaaaa


#[rustc_dummy(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
              aaaaaaaa aaaaaaaa aaaaaaaa)]
#[rustc_dummy[aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
              aaaaaaaa aaaaaaaa aaaaaaaa]]
#[rustc_dummy {
      aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
      aaaaaaaa aaaaaaaa
#[rustc_dummy =
#[rustc_dummy =
  "aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa"]
fn main() { }
------------------------------------------
diff:
------------------------------------------
2 
2 
3 #![feature(rustc_attrs)]
4 
- macro_rules! mac { ($ ($ tt : tt) *) => () }
+ macro_rules! mac { ($($tt : tt) *) => () }
7 mac! {
7 mac! {
8     struct S { field1 : u8, field2 : u16, } impl Clone for S



thread '[pretty] pretty/delimited-token-groups.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2272:9
---- [pretty] pretty/macro.rs stdout ----

error: pretty-printed source does not match expected source
expected:
expected:
------------------------------------------
// pp-exact

#![feature(decl_macro)]

pub(crate) macro mac { ($ arg : expr) => { $ arg + $ arg } }
fn main() { }

------------------------------------------
actual:
actual:
------------------------------------------
// pp-exact

#![feature(decl_macro)]

pub(crate) macro mac { ($arg : expr) => { $arg + $arg } }
fn main() { }

------------------------------------------
diff:
diff:
------------------------------------------
2 
3 #![feature(decl_macro)]
4 
- pub(crate) macro mac { ($ arg : expr) => { $ arg + $ arg } }
+ pub(crate) macro mac { ($arg : expr) => { $arg + $arg } }
7 fn main() { }
8 




thread '[pretty] pretty/macro.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2272:9
---- [pretty] pretty/macro_rules.rs stdout ----

error: pretty-printed source does not match expected source
expected:
expected:
------------------------------------------
// pp-exact

macro_rules! brace { () => { } ; }

macro_rules! bracket[() => { } ;];

macro_rules! paren(() => { } ;);

macro_rules! matcher_brackets {
    (paren) => { } ; (bracket) => { } ; (brace) => { } ;


macro_rules! all_fragments {
    ($ b : block, $ e : expr, $ i : ident, $ it : item, $ l : lifetime, $ lit
     : literal, $ m : meta, $ p : pat, $ pth : path, $ s : stmt, $ tt : tt, $
     ty : ty, $ vis : vis) => { } ;

fn main() { }

------------------------------------------
------------------------------------------
actual:
------------------------------------------
// pp-exact

macro_rules! brace { () => { } ; }

macro_rules! bracket[() => { } ;];

macro_rules! paren(() => { } ;);

macro_rules! matcher_brackets {
    (paren) => { } ; (bracket) => { } ; (brace) => { } ;


macro_rules! all_fragments {
    ($b : block, $e : expr, $i : ident, $it : item, $l : lifetime, $lit :
     literal, $m : meta, $p : pat, $pth : path, $s : stmt, $tt : tt, $ty : ty,
     $vis : vis) => { } ;

fn main() { }

------------------------------------------
------------------------------------------
diff:
------------------------------------------
11 }
12 
13 macro_rules! all_fragments {
-     ($ b : block, $ e : expr, $ i : ident, $ it : item, $ l : lifetime, $ lit
-      : literal, $ m : meta, $ p : pat, $ pth : path, $ s : stmt, $ tt : tt, $
-      ty : ty, $ vis : vis) => { } ;
+     ($b : block, $e : expr, $i : ident, $it : item, $l : lifetime, $lit :
+      literal, $m : meta, $p : pat, $pth : path, $s : stmt, $tt : tt, $ty : ty,
+      $vis : vis) => { } ;
18 
19 fn main() { }




thread '[pretty] pretty/macro_rules.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2272:9

failures:
    [pretty] pretty/cast-lt.rs
    [pretty] pretty/delimited-token-groups.rs
    [pretty] pretty/delimited-token-groups.rs
    [pretty] pretty/macro.rs
    [pretty] pretty/macro_rules.rs

test result: FAILED. 67 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.61s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "pretty" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:09
