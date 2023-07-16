plain

---- [ui] ui/proc-macro/nonterminal-recollect-attr.rs stdout ----
diff of stdout:

7                 span: #0 bytes(276..279),
9         ],
9         ],
-         span: #6 bytes(209..211),
+         span: #3 bytes(209..211),
12     Ident {
13         ident: "struct",


-         span: #6 bytes(212..218),
+         span: #3 bytes(212..218),
16     Ident {
16     Ident {
17         ident: "Foo",

-         span: #6 bytes(219..222),
+         span: #3 bytes(219..222),
20     Group {
21         delimiter: Brace,


22         stream: TokenStream [
23             Ident {
24                 ident: "field",
-                 span: #6 bytes(237..242),
+                 span: #3 bytes(237..242),
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
27             Punct {
28                 ch: ':',


29                 spacing: Alone,
-                 span: #6 bytes(242..243),
+                 span: #3 bytes(242..243),
32             Ident {
32             Ident {
33                 ident: "u8",

-                 span: #6 bytes(244..246),
+                 span: #3 bytes(244..246),
36         ],
36         ],
-         span: #6 bytes(223..256),
+         span: #3 bytes(223..256),
39 ]
40 Second recollected: TokenStream [

44     },
44     },
45     Ident {
46         ident: "struct",
-         span: #6 bytes(212..218),
+         span: #3 bytes(212..218),
49     Ident {
49     Ident {
50         ident: "Foo",

-         span: #6 bytes(219..222),
+         span: #3 bytes(219..222),
53     Group {
54         delimiter: Brace,


55         stream: TokenStream [
56             Ident {
57                 ident: "field",
-                 span: #6 bytes(237..242),
+                 span: #3 bytes(237..242),
60             Punct {
61                 ch: ':',

62                 spacing: Alone,
62                 spacing: Alone,
-                 span: #6 bytes(242..243),
+                 span: #3 bytes(242..243),
65             Ident {
65             Ident {
66                 ident: "u8",

-                 span: #6 bytes(244..246),
+                 span: #3 bytes(244..246),
69         ],
69         ],
-         span: #6 bytes(223..256),
+         span: #3 bytes(223..256),
72 ]
73 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-recollect-attr/nonterminal-recollect-attr.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/nonterminal-recollect-attr.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/nonterminal-recollect-attr.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-recollect-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-recollect-attr/auxiliary"
------------------------------------------
First recollected: TokenStream [
    Group {
        delimiter: None,
        delimiter: None,
        stream: TokenStream [
            Ident {
                ident: "pub",
                span: #0 bytes(276..279),
        ],
        ],
        span: #3 bytes(209..211),
    Ident {
        ident: "struct",
        ident: "struct",
        span: #3 bytes(212..218),
    Ident {
    Ident {
        ident: "Foo",
        span: #3 bytes(219..222),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "field",
                span: #3 bytes(237..242),
            Punct {
                ch: ':',
                spacing: Alone,
                spacing: Alone,
                span: #3 bytes(242..243),
            Ident {
            Ident {
                ident: "u8",
                span: #3 bytes(244..246),
        ],
        ],
        span: #3 bytes(223..256),
]
Second recollected: TokenStream [
    Ident {
    Ident {
        ident: "pub",
        span: #0 bytes(276..279),
    Ident {
        ident: "struct",
        ident: "struct",
        span: #3 bytes(212..218),
    Ident {
    Ident {
        ident: "Foo",
        span: #3 bytes(219..222),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "field",
                span: #3 bytes(237..242),
            Punct {
                ch: ':',
                spacing: Alone,
                spacing: Alone,
                span: #3 bytes(242..243),
            Ident {
            Ident {
                ident: "u8",
                span: #3 bytes(244..246),
        ],
        ],
        span: #3 bytes(223..256),
]

------------------------------------------
stderr:
---
test result: FAILED. 11151 passed; 1 failed; 596 ignored; 0 measured; 0 filtered out; finished in 102.44s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--suite" "ui" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v15.14.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/mir-opt src/test/codegen-units library/core
Build completed unsuccessfully in 0:19:41
