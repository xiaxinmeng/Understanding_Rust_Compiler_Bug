plain

---- [ui] ui/hygiene/unpretty-debug.rs stdout ----
diff of stdout:

8 #![feature /* 0#0 */(no_core)]
9 #![no_core /* 0#0 */]
10 
- macro_rules! foo /* 0#0 */ { ($ x : ident) => { y + $ x } }
+ macro_rules! foo /* 0#0 */ { ($x : ident) => { y + $x } }
12 
13 fn bar /* 0#0 */() {
14     let x /* 0#0 */ = 1;

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/unpretty-debug.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/unpretty-debug.rs`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/unpretty-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunpretty=expanded,hygiene" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/auxiliary"
------------------------------------------
// check-pass
// check-pass
// compile-flags: -Zunpretty=expanded,hygiene

// Don't break whenever Symbol numbering changes
// normalize-stdout-test "\d+#" -> "0#"

// minimal junk
#![feature /* 492#0 */(no_core)]
#![no_core /* 737#0 */]

macro_rules! foo /* 1309#0 */ { ($x : ident) => { y + $x } }

fn bar /* 1312#0 */() {
    let x /* 1310#0 */ = 1;
    y /* 1311#1 */ + x /* 1310#0 */


fn y /* 1311#0 */() { }
/*
Expansions:
Expansions:
0: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
1: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "foo", proc_macro: false }
SyntaxContexts:
SyntaxContexts:
#0: parent: #0, outer_mark: (ExpnId(0), Opaque)
#1: parent: #0, outer_mark: (ExpnId(1), SemiTransparent)

------------------------------------------
stderr:
------------------------------------------
---

30     */ {
31     () =>
32     {
-         meta_macro :: print_def_site! ($ crate :: dummy! ()) ;
+         meta_macro :: print_def_site! ($crate :: dummy! ()) ;
34         // `print_def_site!` will respan the `$crate` identifier
35         // with `Span::def_site()`. This should cause it to resolve
36         // relative to `meta_macro`, *not* `make_macro` (despite

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/meta-macro-hygiene.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/meta-macro-hygiene.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "span-debug" "-Z" "macro-backtrace" "-Z" "unpretty=expanded,hygiene" "-Z" "trim-diagnostic-paths=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/auxiliary"
------------------------------------------
------------------------------------------
Def site: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5)
Input: TokenStream [Ident { ident: "$crate", span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:37: 23:43 (#4) }, Punct { ch: ':', spacing: Joint, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:43: 23:45 (#4) }, Punct { ch: ':', spacing: Alone, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:43: 23:45 (#4) }, Ident { ident: "dummy", span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:45: 23:50 (#4) }, Punct { ch: '!', spacing: Alone, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:50: 23:51 (#4) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:51: 23:53 (#4) }]
Respanned: TokenStream [Ident { ident: "$crate", span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Joint, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Alone, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Ident { ident: "dummy", span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: '!', spacing: Alone, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }]
#![feature /* 492#0 */(prelude_import)]
// aux-build:make-macro.rs
// aux-build:meta-macro.rs
// edition:2018
// compile-flags: -Z span-debug -Z macro-backtrace -Z unpretty=expanded,hygiene -Z trim-diagnostic-paths=no
// check-pass
// normalize-stdout-test "\d+#" -> "0#"
//
// We don't care about symbol ids, so we set them all to 0
// in the stdout

#![no_std /* 751#0 */]
#[prelude_import /* 841#1 */]
use core /* 364#1 */::prelude /* 840#1 */::rust_2018 /* 931#1 */::*;
#[macro_use /* 661#1 */]
extern crate core /* 364#1 */;
#[macro_use /* 661#1 */]
extern crate compiler_builtins /* 315#1 */;
// Don't load unnecessary hygiene information from std
extern crate std /* 1114#0 */;

extern crate meta_macro /* 1309#0 */;
macro_rules! produce_it
    /*
    1310#0
    */ {
    */ {
    () =>
    {
        meta_macro :: print_def_site! ($crate :: dummy! ()) ;
        // `print_def_site!` will respan the `$crate` identifier
        // with `Span::def_site()`. This should cause it to resolve
        // relative to `meta_macro`, *not* `make_macro` (despite
        // the fact that that `print_def_site` is produced by
        // a `macro_rules!` macro in `make_macro`).
}


fn main /* 664#0 */() { ; }
/*
Expansions:
Expansions:
0: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
1: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
2: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "produce_it", proc_macro: false }
3: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
4: parent: ExpnId(2), call_site_ctxt: #4, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "meta_macro::print_def_site", proc_macro: true }
5: parent: ExpnId(4), call_site_ctxt: #5, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "$crate::dummy", proc_macro: true }
SyntaxContexts:
SyntaxContexts:
#0: parent: #0, outer_mark: (ExpnId(0), Opaque)
#1: parent: #0, outer_mark: (ExpnId(1), Opaque)
#2: parent: #0, outer_mark: (ExpnId(1), Transparent)
#3: parent: #0, outer_mark: (ExpnId(3), Opaque)
#4: parent: #0, outer_mark: (ExpnId(2), SemiTransparent)
#5: parent: #0, outer_mark: (ExpnId(4), Opaque)
#6: parent: #4, outer_mark: (ExpnId(4), Transparent)
#7: parent: #0, outer_mark: (ExpnId(4), SemiTransparent)
#8: parent: #0, outer_mark: (ExpnId(5), Opaque)
#9: parent: #5, outer_mark: (ExpnId(5), Transparent)
#10: parent: #5, outer_mark: (ExpnId(5), SemiTransparent)

------------------------------------------
stderr:
------------------------------------------
---
diff of stdout:

50     /*
51     0#0
52     */ {
-     ($ item : item) =>
+     ($item : item) =>
54     {
-         macro inner() { print_bang! { $ item } } inner! () ;
+         macro inner() { print_bang! { $item } } inner! () ;
57     } ;
58 }



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene/nonterminal-token-hygiene.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/nonterminal-token-hygiene.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "span-debug" "-Z" "macro-backtrace" "-Z" "unpretty=expanded,hygiene" "-Z" "trim-diagnostic-paths=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene/auxiliary"
------------------------------------------
------------------------------------------
PRINT-BANG INPUT (DISPLAY): struct S;
PRINT-BANG RE-COLLECTED (DISPLAY): struct S ;
PRINT-BANG INPUT (DEBUG): TokenStream [
    Group {
        delimiter: None,
        stream: TokenStream [
                ident: "struct",
                ident: "struct",
                span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:30:5: 30:11 (#5),
            Ident {
                ident: "S",
                ident: "S",
                span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:30:12: 30:13 (#5),
            Punct {
            Punct {
                ch: ';',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:30:13: 30:14 (#5),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:20:27: 20:32 (#6),
]
]
#![feature /* 492#0 */(prelude_import)]
#![no_std /* 751#0 */]
// Make sure that marks from declarative macros are applied to tokens in nonterminal.
// check-pass
// check-pass
// compile-flags: -Z span-debug -Z macro-backtrace -Z unpretty=expanded,hygiene
// compile-flags: -Z trim-diagnostic-paths=no
// normalize-stdout-test "\d+#" -> "0#"
// aux-build:test-macros.rs

#![feature /* 492#0 */(decl_macro)]

#![no_std /* 751#0 */]
#[prelude_import /* 841#1 */]
use ::core /* 364#1 */::prelude /* 840#1 */::rust_2015 /* 929#1 */::*;
#[macro_use /* 661#1 */]
extern crate core /* 364#2 */;
#[macro_use /* 661#1 */]
extern crate compiler_builtins /* 315#2 */;
// Don't load unnecessary hygiene information from std
extern crate std /* 1114#0 */;

#[macro_use /* 661#0 */]
extern crate test_macros /* 1309#0 */;
macro_rules! outer
    /*
    1310#0
    */ {
    */ {
    ($item : item) =>
    {
        macro inner() { print_bang! { $item } } inner! () ;
    } ;
}


struct S /* 1313#0 */;
macro inner /* 1311#4 */ { () => { print_bang! { struct S; } } }

struct S /* 1313#5 */;
// OK, not a duplicate definition of `S`

fn main /* 664#0 */() { }
/*
Expansions:
Expansions:
0: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
1: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
2: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "outer", proc_macro: false }
3: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
4: parent: ExpnId(2), call_site_ctxt: #4, def_site_ctxt: #4, kind: Macro { kind: Bang, name: "inner", proc_macro: false }
5: parent: ExpnId(4), call_site_ctxt: #6, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "print_bang", proc_macro: true }
SyntaxContexts:
SyntaxContexts:
#0: parent: #0, outer_mark: (ExpnId(0), Opaque)
#1: parent: #0, outer_mark: (ExpnId(1), Opaque)
#2: parent: #0, outer_mark: (ExpnId(1), Transparent)
#3: parent: #0, outer_mark: (ExpnId(3), Opaque)
#4: parent: #0, outer_mark: (ExpnId(2), SemiTransparent)
#5: parent: #0, outer_mark: (ExpnId(4), Opaque)
#6: parent: #4, outer_mark: (ExpnId(4), Opaque)
#7: parent: #0, outer_mark: (ExpnId(5), Opaque)
#8: parent: #6, outer_mark: (ExpnId(5), Transparent)
#9: parent: #5, outer_mark: (ExpnId(5), SemiTransparent)

------------------------------------------
stderr:
------------------------------------------
---
test result: FAILED. 11943 passed; 3 failed; 97 ignored; 0 measured; 0 filtered out; finished in 136.52s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:52
