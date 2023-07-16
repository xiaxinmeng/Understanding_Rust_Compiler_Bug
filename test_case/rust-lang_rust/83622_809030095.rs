plain
.................................................................................................... 9400/11718
.................................................................................................... 9500/11718
............................................................i........i.............................. 9600/11718
.................................................................................................... 9700/11718
.......iiiiiii.iiiiii.i............................................................................. 9800/11718
.................................................................................................... 10000/11718
.................................................................................................... 10100/11718
.................................................................................................... 10200/11718
.................................................................................................... 10300/11718
---
diff of stderr:

2   --> $DIR/old-closure-expr-precedence.rs:60:21
3    |
4 LL |   if (true) { 12; };;; -num;
-    |                     ^^ help: remove these semicolons
6    |
7    = note: `#[warn(redundant_semicolons)]` on by default
+ help: remove these semicolons
+    |
+    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL |   if (true) { 12; }; -num;
8 
9 warning: 1 warning emitted
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/old-closure-expr-precedence/old-closure-expr-precedence.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/old-closure-expr-precedence.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/old-closure-expr-precedence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/old-closure-expr-precedence/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/old-closure-expr-precedence/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unnecessary trailing semicolons
  --> /checkout/src/test/ui/closures/old-closure-expr-precedence.rs:60:21
   |
LL |   if (true) { 12; };;; -num;
   |
   = note: `#[warn(redundant_semicolons)]` on by default
help: remove these semicolons
   |
   |
LL |   if (true) { 12; }; -num;

warning: 1 warning emitted


---
7 note: the lint level is defined here
8   --> $DIR/item-stmt-semi.rs:1:9

9    |
10 LL | #![deny(redundant_semicolons)]
+ help: remove this semicolon
+    |
+ LL |     fn inner() {}
+    |                 --
---
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/item-stmt-semi/item-stmt-semi.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/redundant-semicolon/item-stmt-semi.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/redundant-semicolon/item-stmt-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/item-stmt-semi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/item-stmt-semi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unnecessary trailing semicolon
  --> /checkout/src/test/ui/lint/redundant-semicolon/item-stmt-semi.rs:4:18
   |
LL |     fn inner() {}; //~ ERROR unnecessary
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/redundant-semicolon/item-stmt-semi.rs:1:9
   |
   |
LL | #![deny(redundant_semicolons)]
help: remove this semicolon
   |
   |
LL |     fn inner() {} //~ ERROR unnecessary

error: unnecessary trailing semicolon
  --> /checkout/src/test/ui/lint/redundant-semicolon/item-stmt-semi.rs:5:18
   |
   |
LL |     struct Bar {}; //~ ERROR unnecessary
   |
help: remove this semicolon
   |
   |
LL |     struct Bar {} //~ ERROR unnecessary

error: aborting due to 2 previous errors


---
8 note: the lint level is defined here
9   --> $DIR/redundant-semi-proc-macro.rs:3:9

10    |
11 LL | #![deny(redundant_semicolons)]
+ help: remove this semicolon
+    |
+ LL |     let tst = 123;
+    |                  --
---
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro/redundant-semi-proc-macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/redundant-semicolon/redundant-semi-proc-macro.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
TokenStream [Ident { ident: "fn", span: #0 bytes(198..200) }, Ident { ident: "span_preservation", span: #0 bytes(201..218) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(218..220) }, Group { delimiter: Brace, stream: TokenStream [Ident { ident: "let", span: #0 bytes(228..231) }, Ident { ident: "tst", span: #0 bytes(232..235) }, Punct { ch: '=', spacing: Alone, span: #0 bytes(236..237) }, Literal { kind: Integer, symbol: "123", suffix: None, span: #0 bytes(238..241) }, Punct { ch: ';', spacing: Joint, span: #0 bytes(241..242) }, Punct { ch: ';', spacing: Alone, span: #0 bytes(242..243) }, Ident { ident: "match", span: #0 bytes(289..294) }, Ident { ident: "tst", span: #0 bytes(295..298) }, Group { delimiter: Brace, stream: TokenStream [Literal { kind: Integer, symbol: "123", suffix: None, span: #0 bytes(483..486) }, Punct { ch: '=', spacing: Joint, span: #0 bytes(487..489) }, Punct { ch: '>', spacing: Alone, span: #0 bytes(487..489) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(490..492) }, Punct { ch: ',', spacing: Alone, span: #0 bytes(492..493) }, Ident { ident: "_", span: #0 bytes(502..503) }, Punct { ch: '=', spacing: Joint, span: #0 bytes(504..506) }, Punct { ch: '>', spacing: Alone, span: #0 bytes(504..506) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(507..509) }], span: #0 bytes(299..515) }, Punct { ch: ';', spacing: Joint, span: #0 bytes(515..516) }, Punct { ch: ';', spacing: Joint, span: #0 bytes(516..517) }, Punct { ch: ';', spacing: Alone, span: #0 bytes(517..518) }], span: #0 bytes(222..562) }]
  --> /checkout/src/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs:9:19
   |
   |
LL |     let tst = 123;; //~ ERROR unnecessary trailing semicolon
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs:3:9
   |
   |
LL | #![deny(redundant_semicolons)]
help: remove this semicolon
   |
   |
LL |     let tst = 123; //~ ERROR unnecessary trailing semicolon

error: unnecessary trailing semicolons
  --> /checkout/src/test/ui/lint/redundant-semicolon/redundant-semi-proc-macro.rs:16:7
   |
   |
LL |     };;; //~ ERROR unnecessary trailing semicolons
   |
help: remove these semicolons
   |
   |
LL |     }; //~ ERROR unnecessary trailing semicolons

error: aborting due to 2 previous errors


---
test result: FAILED. 11619 passed; 3 failed; 96 ignored; 0 measured; 0 filtered out; finished in 134.23s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:21
