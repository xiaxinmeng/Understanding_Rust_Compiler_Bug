plain
.................................................................................................... 8400/11462
.................................................................................................... 8500/11462
.................................................................................................... 8600/11462
.................................................................................................... 8700/11462
....................iiii.iiii......................................................i......F.......i. 8800/11462
............F....................................................................................... 8900/11462
.................................................................................................... 9100/11462
.................................................................................................... 9200/11462
.................................................................................................... 9300/11462
.................................................................................................... 9400/11462
.................................................................................................... 9400/11462
....................i......i.F...................................................................... 9500/11462
...........................................................iiiiiii..iiiiii.i........................ 9600/11462
.................................................................................................... 9800/11462
.................................................................................................... 9900/11462
.................................................................................................... 10000/11462
.................................................................................................... 10100/11462
---
failures:

---- [ui] ui/hygiene/unpretty-debug.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/unpretty-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunpretty=expanded,hygiene" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0601]: `main` function not found in crate `unpretty_debug`
   |
   |
LL | / #![feature(no_core)]
LL | | #![no_core]
LL | |
LL | | macro_rules! foo {
LL | |
LL | |
LL | | fn y() {}
   | |_________^ consider adding a `main` function to `/checkout/src/test/ui/hygiene/unpretty-debug.rs`

error: requires `sized` lang_item
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0601`.


------------------------------------------


---- [ui] ui/issues/issue-60662.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60662.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=hir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: could not find defining uses
  --> /checkout/src/test/ui/issues/issue-60662.rs:10:25
   |
LL |     pub type ServeFut = impl Animal;

error: aborting due to previous error



------------------------------------------


---- [ui] ui/proc-macro/meta-macro-hygiene.rs stdout ----
diff of stdout:

1 Def site: $DIR/auxiliary/make-macro.rs:7:9: 7:56 (#5)
2 Input: TokenStream [Ident { ident: "$crate", span: $DIR/meta-macro-hygiene.rs:24:37: 24:43 (#4) }, Punct { ch: ':', spacing: Joint, span: $DIR/meta-macro-hygiene.rs:24:43: 24:45 (#4) }, Punct { ch: ':', spacing: Alone, span: $DIR/meta-macro-hygiene.rs:24:43: 24:45 (#4) }, Ident { ident: "dummy", span: $DIR/meta-macro-hygiene.rs:24:45: 24:50 (#4) }, Punct { ch: '!', spacing: Alone, span: $DIR/meta-macro-hygiene.rs:24:50: 24:51 (#4) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: $DIR/meta-macro-hygiene.rs:24:51: 24:53 (#4) }]
3 Respanned: TokenStream [Ident { ident: "$crate", span: $DIR/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Joint, span: $DIR/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Alone, span: $DIR/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Ident { ident: "dummy", span: $DIR/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: '!', spacing: Alone, span: $DIR/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: $DIR/auxiliary/make-macro.rs:7:9: 7:56 (#5) }]
- #![feature /* 0#0 */(prelude_import)]
- // ignore-tidy-linelength
- // aux-build:make-macro.rs
- // aux-build:meta-macro.rs
- // edition:2018
- // compile-flags: -Z span-debug -Z macro-backtrace -Z unpretty=expanded,hygiene -Z trim-diagnostic-paths=no
- // check-pass
- // normalize-stdout-test "\d+#" -> "0#"
- //
- // We don't care about symbol ids, so we set them all to 0
- // in the stdout
- 
- #![no_std /* 0#0 */]
- #[prelude_import /* 0#1 */]
- use core /* 0#1 */::prelude /* 0#1 */::v1 /* 0#1 */::*;
- #[macro_use /* 0#1 */]
- extern crate core /* 0#1 */;
- #[macro_use /* 0#1 */]
- extern crate compiler_builtins /* 0#1 */;
- // Don't load unnecessary hygiene information from std
- extern crate std /* 0#0 */;
- 
- extern crate meta_macro /* 0#0 */;
- 
- macro_rules! produce_it
-     0#0
-     */ {
-     () =>
-     {
-     {
-         meta_macro :: print_def_site ! ($ crate :: dummy ! ()) ;
-         // `print_def_site!` will respan the `$crate` identifier
-         // with `Span::def_site()`. This should cause it to resolve
-         // relative to `meta_macro`, *not* `make_macro` (despite
-         // the fact that that `print_def_site` is produced by
-         // a `macro_rules!` macro in `make_macro`).
- }
- 
- 
- fn main /* 0#0 */() { ; }
- /*
- Expansions:
- Expansions:
- 0: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
- 1: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
- 2: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "produce_it")
- 3: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
- 4: parent: ExpnId(2), call_site_ctxt: #4, def_site_ctxt: #0, kind: Macro(Bang, "meta_macro::print_def_site")
- 5: parent: ExpnId(4), call_site_ctxt: #5, def_site_ctxt: #0, kind: Macro(Bang, "$crate::dummy")
- SyntaxContexts:
- SyntaxContexts:
- #0: parent: #0, outer_mark: (ExpnId(0), Opaque)
- #1: parent: #0, outer_mark: (ExpnId(1), Opaque)
- #2: parent: #0, outer_mark: (ExpnId(1), Transparent)
- #3: parent: #0, outer_mark: (ExpnId(3), Opaque)
- #4: parent: #0, outer_mark: (ExpnId(2), SemiTransparent)
- #5: parent: #0, outer_mark: (ExpnId(4), Opaque)
- #6: parent: #4, outer_mark: (ExpnId(4), Transparent)
- #7: parent: #0, outer_mark: (ExpnId(4), SemiTransparent)
- #8: parent: #0, outer_mark: (ExpnId(5), Opaque)
- #9: parent: #5, outer_mark: (ExpnId(5), Transparent)
- #10: parent: #5, outer_mark: (ExpnId(5), SemiTransparent)
67 


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/meta-macro-hygiene.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/meta-macro-hygiene.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "span-debug" "-Z" "macro-backtrace" "-Z" "unpretty=expanded,hygiene" "-Z" "trim-diagnostic-paths=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/auxiliary"
------------------------------------------
------------------------------------------
Def site: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5)
Input: TokenStream [Ident { ident: "$crate", span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:37: 24:43 (#4) }, Punct { ch: ':', spacing: Joint, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:43: 24:45 (#4) }, Punct { ch: ':', spacing: Alone, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:43: 24:45 (#4) }, Ident { ident: "dummy", span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:45: 24:50 (#4) }, Punct { ch: '!', spacing: Alone, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:50: 24:51 (#4) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:24:51: 24:53 (#4) }]
Respanned: TokenStream [Ident { ident: "$crate", span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Joint, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Alone, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Ident { ident: "dummy", span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: '!', spacing: Alone, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }]
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/proc-macro/nonterminal-token-hygiene.rs stdout ----
diff of stdout:

21         span: $DIR/nonterminal-token-hygiene.rs:20:27: 20:32 (#6),
23 ]
23 ]
- #![feature /* 0#0 */(prelude_import)]
- #![no_std /* 0#0 */]
- // Make sure that marks from declarative macros are applied to tokens in nonterminal.
- // check-pass
- // check-pass
- // compile-flags: -Z span-debug -Z macro-backtrace -Z unpretty=expanded,hygiene
- // compile-flags: -Z trim-diagnostic-paths=no
- // normalize-stdout-test "\d+#" -> "0#"
- // aux-build:test-macros.rs
- 
- #![feature /* 0#0 */(decl_macro)]
- 
- #![no_std /* 0#0 */]
- #[prelude_import /* 0#1 */]
- use ::core /* 0#1 */::prelude /* 0#1 */::v1 /* 0#1 */::*;
- #[macro_use /* 0#1 */]
- extern crate core /* 0#2 */;
- #[macro_use /* 0#1 */]
- extern crate compiler_builtins /* 0#2 */;
- // Don't load unnecessary hygiene information from std
- extern crate std /* 0#0 */;
- 
- #[macro_use /* 0#0 */]
- extern crate test_macros /* 0#0 */;
- macro_rules! outer
-     /*
-     0#0
-     */ {
-     */ {
-     ($ item : item) =>
-     {
-         macro inner() { print_bang ! { $ item } } inner ! () ;
-     } ;
- }
- 
- 
- struct S /* 0#0 */;
- macro inner /* 0#4 */ { () => { print_bang ! { struct S; } } }
- 
- struct S /* 0#5 */;
- // OK, not a duplicate definition of `S`
- 
- fn main /* 0#0 */() { }
- /*
- Expansions:
- Expansions:
- 0: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
- 1: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
- 2: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "outer")
- 3: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
- 4: parent: ExpnId(2), call_site_ctxt: #4, def_site_ctxt: #4, kind: Macro(Bang, "inner")
- 5: parent: ExpnId(4), call_site_ctxt: #6, def_site_ctxt: #0, kind: Macro(Bang, "print_bang")
- SyntaxContexts:
- SyntaxContexts:
- #0: parent: #0, outer_mark: (ExpnId(0), Opaque)
- #1: parent: #0, outer_mark: (ExpnId(1), Opaque)
- #2: parent: #0, outer_mark: (ExpnId(1), Transparent)
- #3: parent: #0, outer_mark: (ExpnId(3), Opaque)
- #4: parent: #0, outer_mark: (ExpnId(2), SemiTransparent)
- #5: parent: #0, outer_mark: (ExpnId(4), Opaque)
- #6: parent: #4, outer_mark: (ExpnId(4), Opaque)
- #7: parent: #0, outer_mark: (ExpnId(5), Opaque)
- #8: parent: #6, outer_mark: (ExpnId(5), Transparent)
- #9: parent: #5, outer_mark: (ExpnId(5), SemiTransparent)
89 


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene/nonterminal-token-hygiene.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/nonterminal-token-hygiene.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "span-debug" "-Z" "macro-backtrace" "-Z" "unpretty=expanded,hygiene" "-Z" "trim-diagnostic-paths=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene/auxiliary"
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

------------------------------------------
stderr:
---

---- [ui] ui/rfc-2497-if-let-chains/ast-pretty-check.rs stdout ----
diff of stdout:

- #![feature(prelude_import)]
- #![no_std]
- #[prelude_import]
- use ::std::prelude::v1::*;
- #[macro_use]
- extern crate std;
- // build-pass (FIXME(62277): could be check-pass?)
- // compile-flags: -Z unpretty=expanded
- 
- fn main() { if let 0 = 1 { } }


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/ast-pretty-check.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/ast-pretty-check.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/ast-pretty-check.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/ast-pretty-check.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=expanded" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 11364 passed; 5 failed; 93 ignored; 0 measured; 0 filtered out; finished in 114.88s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:42
