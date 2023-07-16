plain

---- [ui] ui/proc-macro/meta-macro-hygiene.rs stdout ----
diff of stdout:

49 crate0::{{expn2}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "produce_it")
50 crate0::{{expn3}}: parent: crate0::{{expn2}}, call_site_ctxt: #4, def_site_ctxt: #0, kind: Macro(Bang, "meta_macro::print_def_site")
51 crate0::{{expn4}}: parent: crate0::{{expn3}}, call_site_ctxt: #5, def_site_ctxt: #0, kind: Macro(Bang, "$crate::dummy")
- crate1::{{expnNNNN}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "include")
53 crate2::{{expn1}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
+ crate1::{{expnNNNN}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "include")
55 SyntaxContexts:
55 SyntaxContexts:
56 #0: parent: #0, outer_mark: (crate0::{{expn0}}, Opaque)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/meta-macro-hygiene.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/meta-macro-hygiene.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/meta-macro-hygiene.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-Z" "span-debug" "-Z" "macro-backtrace" "-Z" "unpretty=expanded,hygiene" "-Z" "trim-diagnostic-paths=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/auxiliary"
------------------------------------------
------------------------------------------
Def site: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5)
Input: TokenStream [Ident { ident: "$crate", span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:37: 24:43 (#4) }, Punct { ch: ':', spacing: Joint, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:43: 24:45 (#4) }, Punct { ch: ':', spacing: Alone, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:43: 24:45 (#4) }, Ident { ident: "dummy", span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:45: 24:50 (#4) }, Punct { ch: '!', spacing: Alone, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:50: 24:51 (#4) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:51: 24:53 (#4) }]
Respanned: TokenStream [Ident { ident: "$crate", span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Joint, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Alone, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Ident { ident: "dummy", span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: '!', spacing: Alone, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }]
#![feature /* 549#0 */(prelude_import)]
// aux-build:make-macro.rs
// aux-build:meta-macro.rs
// edition:2018
// compile-flags: -Z span-debug -Z macro-backtrace -Z unpretty=expanded,hygiene -Z trim-diagnostic-paths=no
// check-pass
// normalize-stdout-test "\d+#" -> "0#"
// normalize-stdout-test "expn\d{4,}" -> "expnNNNN"
//
// We don't care about symbol ids, so we set them all to 0


#![no_std /* 824#0 */]
#[prelude_import /* 916#1 */]
use core /* 416#1 */::prelude /* 915#1 */::rust_2018 /* 1007#1 */::*;
#[macro_use /* 726#1 */]
extern crate core /* 416#1 */;
#[macro_use /* 726#1 */]
extern crate compiler_builtins /* 362#1 */;
// Don't load unnecessary hygiene information from std
extern crate std /* 1192#0 */;

extern crate meta_macro /* 1392#0 */;
macro_rules! produce_it
    /*
    1393#0
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


fn main /* 729#0 */() { ; }
/*
Expansions:
Expansions:
crate0::{{expn0}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
crate0::{{expn1}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
crate0::{{expn2}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "produce_it")
crate0::{{expn3}}: parent: crate0::{{expn2}}, call_site_ctxt: #4, def_site_ctxt: #0, kind: Macro(Bang, "meta_macro::print_def_site")
crate0::{{expn4}}: parent: crate0::{{expn3}}, call_site_ctxt: #5, def_site_ctxt: #0, kind: Macro(Bang, "$crate::dummy")
crate2::{{expn1}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
crate1::{{expn2376}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "include")
SyntaxContexts:
SyntaxContexts:
#0: parent: #0, outer_mark: (crate0::{{expn0}}, Opaque)
#1: parent: #0, outer_mark: (crate0::{{expn1}}, Opaque)
#2: parent: #0, outer_mark: (crate0::{{expn1}}, Transparent)
#3: parent: #0, outer_mark: (crate2::{{expn1}}, Opaque)
#4: parent: #0, outer_mark: (crate0::{{expn2}}, SemiTransparent)
#5: parent: #0, outer_mark: (crate0::{{expn3}}, Opaque)
#6: parent: #4, outer_mark: (crate0::{{expn3}}, Transparent)
#7: parent: #0, outer_mark: (crate0::{{expn3}}, SemiTransparent)
#8: parent: #0, outer_mark: (crate0::{{expn4}}, Opaque)
#9: parent: #5, outer_mark: (crate0::{{expn4}}, Transparent)
#10: parent: #5, outer_mark: (crate0::{{expn4}}, SemiTransparent)

------------------------------------------
stderr:
------------------------------------------
------------------------------------------

------------------------------------------


---- [ui] ui/proc-macro/nonterminal-token-hygiene.rs stdout ----
diff of stdout:

73 crate0::{{expn2}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "outer")
74 crate0::{{expn3}}: parent: crate0::{{expn2}}, call_site_ctxt: #4, def_site_ctxt: #4, kind: Macro(Bang, "inner")
75 crate0::{{expn4}}: parent: crate0::{{expn3}}, call_site_ctxt: #6, def_site_ctxt: #0, kind: Macro(Bang, "print_bang")
- crate1::{{expnNNNN}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "include")
77 crate2::{{expn1}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
+ crate1::{{expnNNNN}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "include")
79 SyntaxContexts:
79 SyntaxContexts:
80 #0: parent: #0, outer_mark: (crate0::{{expn0}}, Opaque)

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene/nonterminal-token-hygiene.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/nonterminal-token-hygiene.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-Z" "span-debug" "-Z" "macro-backtrace" "-Z" "unpretty=expanded,hygiene" "-Z" "trim-diagnostic-paths=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene/auxiliary"
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
                span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:31:5: 31:11 (#5),
            Ident {
                ident: "S",
                ident: "S",
                span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:31:12: 31:13 (#5),
            Punct {
            Punct {
                ch: ';',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:31:13: 31:14 (#5),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:21:27: 21:32 (#6),
]
]
#![feature /* 549#0 */(prelude_import)]
#![no_std /* 824#0 */]
// Make sure that marks from declarative macros are applied to tokens in nonterminal.
// check-pass
// check-pass
// compile-flags: -Z span-debug -Z macro-backtrace -Z unpretty=expanded,hygiene
// compile-flags: -Z trim-diagnostic-paths=no
// normalize-stdout-test "\d+#" -> "0#"
// normalize-stdout-test "expn\d{4,}" -> "expnNNNN"
// aux-build:test-macros.rs

#![feature /* 549#0 */(decl_macro)]

#![no_std /* 824#0 */]
#[prelude_import /* 916#1 */]
use ::core /* 416#1 */::prelude /* 915#1 */::rust_2015 /* 1005#1 */::*;
#[macro_use /* 726#1 */]
extern crate core /* 416#2 */;
#[macro_use /* 726#1 */]
extern crate compiler_builtins /* 362#2 */;
// Don't load unnecessary hygiene information from std
extern crate std /* 1192#0 */;

#[macro_use /* 726#0 */]
extern crate test_macros /* 1392#0 */;
macro_rules! outer
    /*
    1393#0
    */ {
    */ {
    ($item : item) =>
    {
        macro inner() { print_bang! { $item } } inner! () ;
    } ;
}


struct S /* 1396#0 */;
macro inner /* 1394#4 */ { () => { print_bang! { struct S; } } }

struct S /* 1396#5 */;
// OK, not a duplicate definition of `S`

fn main /* 729#0 */() { }
/*
Expansions:
Expansions:
crate0::{{expn0}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
crate0::{{expn1}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
crate0::{{expn2}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "outer")
crate0::{{expn3}}: parent: crate0::{{expn2}}, call_site_ctxt: #4, def_site_ctxt: #4, kind: Macro(Bang, "inner")
crate0::{{expn4}}: parent: crate0::{{expn3}}, call_site_ctxt: #6, def_site_ctxt: #0, kind: Macro(Bang, "print_bang")
crate2::{{expn1}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
crate1::{{expn2376}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "include")
SyntaxContexts:
SyntaxContexts:
#0: parent: #0, outer_mark: (crate0::{{expn0}}, Opaque)
#1: parent: #0, outer_mark: (crate0::{{expn1}}, Opaque)
#2: parent: #0, outer_mark: (crate0::{{expn1}}, Transparent)
#3: parent: #0, outer_mark: (crate2::{{expn1}}, Opaque)
#4: parent: #0, outer_mark: (crate0::{{expn2}}, SemiTransparent)
#5: parent: #0, outer_mark: (crate0::{{expn3}}, Opaque)
#6: parent: #4, outer_mark: (crate0::{{expn3}}, Opaque)
#7: parent: #0, outer_mark: (crate0::{{expn4}}, Opaque)
#8: parent: #6, outer_mark: (crate0::{{expn4}}, Transparent)
#9: parent: #5, outer_mark: (crate0::{{expn4}}, SemiTransparent)

------------------------------------------
stderr:
------------------------------------------
---
test result: FAILED. 12048 passed; 2 failed; 162 ignored; 0 measured; 0 filtered out; finished in 78.75s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:20
