plain

---- [ui] ui/proc-macro/macro-rules-derive-cfg.rs stdout ----
diff of stdout:

10 PRINT-DERIVE INPUT (DEBUG): TokenStream [
12         ident: "struct",
12         ident: "struct",
-         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
15     Ident {
15     Ident {
16         ident: "Foo",

-         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
19     Group {
20         delimiter: Brace,


21         stream: TokenStream [
22             Ident {
23                 ident: "val",
-                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
26             Punct {
27                 ch: ':',

28                 spacing: Alone,
28                 spacing: Alone,
-                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
31             Group {
32                 delimiter: Bracket,


33                 stream: TokenStream [
34                     Ident {
35                         ident: "bool",
-                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
38                     Punct {
38                     Punct {
39                         ch: ';',
40                         spacing: Alone,
40                         spacing: Alone,
-                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
43                     Group {
44                         delimiter: Brace,


45                         stream: TokenStream [
46                             Ident {
47                                 ident: "let",
-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
50                             Ident {
51                                 ident: "a",


-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
54                             Punct {
54                             Punct {
55                                 ch: '=',
56                                 spacing: Alone,
56                                 spacing: Alone,
-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
59                             Punct {
59                             Punct {
60                                 ch: '#',
61                                 spacing: Alone,
61                                 spacing: Alone,
-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
64                             Group {
65                                 delimiter: Bracket,


66                                 stream: TokenStream [
67                                     Ident {
68                                         ident: "rustc_dummy",
-                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
71                                     Group {
72                                         delimiter: Parenthesis,


73                                         stream: TokenStream [
74                                             Ident {
75                                                 ident: "first",
-                                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
78                                         ],
78                                         ],
-                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
81                                 ],
81                                 ],
-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
84                             Punct {
84                             Punct {
85                                 ch: '#',
86                                 spacing: Alone,
86                                 spacing: Alone,
-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
89                             Group {
90                                 delimiter: Bracket,


91                                 stream: TokenStream [
92                                     Ident {
93                                         ident: "rustc_dummy",
-                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
96                                     Group {
97                                         delimiter: Parenthesis,


98                                         stream: TokenStream [
99                                             Ident {
100                                                 ident: "second",
-                                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
103                                         ],
103                                         ],
-                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
106                                 ],
106                                 ],
-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
109                             Group {
110                                 delimiter: Brace,

112                                     Punct {
112                                     Punct {
113                                         ch: '#',
114                                         spacing: Joint,
-                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
117                                     Punct {
117                                     Punct {
118                                         ch: '!',
119                                         spacing: Alone,
119                                         spacing: Alone,
-                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
122                                     Group {
123                                         delimiter: Bracket,


124                                         stream: TokenStream [
126                                                 ident: "allow",
126                                                 ident: "allow",
-                                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
129                                             Group {
130                                                 delimiter: Parenthesis,


131                                                 stream: TokenStream [
133                                                         ident: "unused",
133                                                         ident: "unused",
-                                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
136                                                 ],
136                                                 ],
-                                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
139                                         ],
139                                         ],
-                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
142                                     Literal {
143                                         kind: Integer,

144                                         symbol: "30",
144                                         symbol: "30",
145                                         suffix: None,
-                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
148                                 ],
148                                 ],
-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
151                             Punct {
151                             Punct {
152                                 ch: ';',
153                                 spacing: Alone,
153                                 spacing: Alone,
-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
156                             Literal {
157                                 kind: Integer,

158                                 symbol: "0",
158                                 symbol: "0",
159                                 suffix: None,
-                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
162                         ],
162                         ],
-                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
165                 ],
165                 ],
-                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
168             Punct {
168             Punct {
169                 ch: ',',
170                 spacing: Alone,
170                 spacing: Alone,
-                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+                 span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
173         ],
173         ],
-         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#6),
+         span: $DIR/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
176 ]
177 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/macro-rules-derive-cfg/macro-rules-derive-cfg.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/macro-rules-derive-cfg.rs`
error: 1 errors occurred comparing output.
error: 1 errors occurred comparing output.
failed to decode compiler output as json: line: {
output: PRINT-DERIVE INPUT (DISPLAY): struct Foo
    val :
    val :
    [bool ;
     {
         let a = #[rustc_dummy(first)] #[rustc_dummy(second)]
         { # ! [allow(unused)] 30 } ; 0
     }],
}
PRINT-DERIVE INPUT (DEBUG): TokenStream [
        ident: "struct",
        ident: "struct",
        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
    Ident {
    Ident {
        ident: "Foo",
        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "val",
                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
            Punct {
                ch: ':',
                spacing: Alone,
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "bool",
                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                    Punct {
                    Punct {
                        ch: ';',
                        spacing: Alone,
                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                    Group {
                        delimiter: Brace,
                        delimiter: Brace,
                        stream: TokenStream [
                            Ident {
                                ident: "let",
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                            Ident {
                                ident: "a",
                                ident: "a",
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                            Punct {
                            Punct {
                                ch: '=',
                                spacing: Alone,
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                            Punct {
                            Punct {
                                ch: '#',
                                spacing: Alone,
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                            Group {
                                delimiter: Bracket,
                                delimiter: Bracket,
                                stream: TokenStream [
                                    Ident {
                                        ident: "rustc_dummy",
                                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                    Group {
                                        delimiter: Parenthesis,
                                        delimiter: Parenthesis,
                                        stream: TokenStream [
                                            Ident {
                                                ident: "first",
                                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                        ],
                                        ],
                                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                ],
                                ],
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                            Punct {
                            Punct {
                                ch: '#',
                                spacing: Alone,
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                            Group {
                                delimiter: Bracket,
                                delimiter: Bracket,
                                stream: TokenStream [
                                    Ident {
                                        ident: "rustc_dummy",
                                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                    Group {
                                        delimiter: Parenthesis,
                                        delimiter: Parenthesis,
                                        stream: TokenStream [
                                            Ident {
                                                ident: "second",
                                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                        ],
                                        ],
                                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                ],
                                ],
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                            Group {
                                delimiter: Brace,
                                delimiter: Brace,
                                stream: TokenStream [
                                    Punct {
                                        ch: '#',
                                        spacing: Joint,
                                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                    Punct {
                                    Punct {
                                        ch: '!',
                                        spacing: Alone,
                                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                    Group {
                                        delimiter: Bracket,
                                        delimiter: Bracket,
                                        stream: TokenStream [
                                                ident: "allow",
                                                ident: "allow",
                                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                            Group {
                                                delimiter: Parenthesis,
                                                delimiter: Parenthesis,
                                                stream: TokenStream [
                                                        ident: "unused",
                                                        ident: "unused",
                                                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                                ],
                                                ],
                                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                        ],
                                        ],
                                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                    Literal {
                                        kind: Integer,
                                        symbol: "30",
                                        suffix: None,
                                        suffix: None,
                                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                                ],
                                ],
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                            Punct {
                            Punct {
                                ch: ';',
                                spacing: Alone,
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                            Literal {
                                kind: Integer,
                                symbol: "0",
                                suffix: None,
                                suffix: None,
                                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
            Punct {
            Punct {
                ch: ',',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/macro-rules-derive-cfg.rs:14:9: 19:10 (#3),
]
thread '[ui] ui/proc-macro/macro-rules-derive-cfg.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:126:21


---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--suite" "ui" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v14.4.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.1-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/mir-opt src/test/codegen-units library/core
Build completed unsuccessfully in 0:18:52
