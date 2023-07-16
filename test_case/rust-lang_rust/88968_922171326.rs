plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu

failures:

---- [mir-opt] mir-opt/coverage_graphviz.rs stdout ----
2     graph [fontname="Courier, monospace"];
3     node [fontname="Courier, monospace"];
4     edge [fontname="Courier, monospace"];
-     bcb2__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb2</td></tr><tr><td align="left" balign="left">Expression(bcb0 - bcb1) at 13:10-13:10<br/>    13:10-13:10: @4[0]: Coverage::Expression(4294967295) = 1 - 2 for $DIR/coverage_graphviz.rs:13:10 - 13:11</td></tr><tr><td align="left" balign="left">bb4: Goto</td></tr></table>>];
-     bcb1__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb1</td></tr><tr><td align="left" balign="left">Counter(bcb1) at 12:13-12:18<br/>    12:13-12:18: @3[0]: Coverage::Expression(4294967294) = 2 + 0 for $DIR/coverage_graphviz.rs:15:1 - 15:2<br/>Expression(bcb1 + 0) at 15:2-15:2<br/>    15:2-15:2: @3.Return: return</td></tr><tr><td align="left" balign="left">bb3: Return</td></tr></table>>];
-     bcb0__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb0</td></tr><tr><td align="left" balign="left"></td></tr><tr><td align="left" balign="left">Counter(bcb0) at 9:1-11:17<br/>    11:12-11:17: @1.Call: _2 = bar() -&gt; [return: bb2, unwind: bb5]</td></tr><tr><td align="left" balign="left">bb0: FalseUnwind<br/>bb1: Call</td></tr><tr><td align="left" balign="left">bb2: SwitchInt</td></tr></table>>];
-     bcb2__Cov_0_3 -> bcb0__Cov_0_3 [label=<>];
-     bcb0__Cov_0_3 -> bcb2__Cov_0_3 [label=<false>];
-     bcb0__Cov_0_3 -> bcb1__Cov_0_3 [label=<otherwise>];
+     bcb3__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb3</td></tr><tr><td align="left" balign="left">Counter(bcb3) at 13:10-13:10<br/>    13:10-13:10: @5[0]: Coverage::Counter(2) for $DIR/coverage_graphviz.rs:13:10 - 13:11</td></tr><tr><td align="left" balign="left">bb5: Goto</td></tr></table>>];
+     bcb2__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb2</td></tr><tr><td align="left" balign="left">Expression(bcb1:(bcb0 + bcb3) - bcb3) at 12:13-12:18<br/>    12:13-12:18: @4[0]: Coverage::Expression(4294967293) = 4294967294 + 0 for $DIR/coverage_graphviz.rs:15:1 - 15:2<br/>Expression(bcb2:(bcb1:(bcb0 + bcb3) - bcb3) + 0) at 15:2-15:2<br/>    15:2-15:2: @4.Return: return</td></tr><tr><td align="left" balign="left">bb4: Return</td></tr></table>>];
+     bcb1__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb1</td></tr><tr><td align="left" balign="left">Expression(bcb0 + bcb3) at 10:5-11:17<br/>    11:12-11:17: @2.Call: _2 = bar() -&gt; [return: bb3, unwind: bb6]</td></tr><tr><td align="left" balign="left">bb1: FalseUnwind<br/>bb2: Call</td></tr><tr><td align="left" balign="left">bb3: SwitchInt</td></tr></table>>];
+     bcb0__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb0</td></tr><tr><td align="left" balign="left"></td></tr><tr><td align="left" balign="left">Counter(bcb0) at 9:1-9:11<br/>    </td></tr><tr><td align="left" balign="left">bb0: Goto</td></tr></table>>];
+     bcb3__Cov_0_3 -> bcb1__Cov_0_3 [label=<>];
+     bcb1__Cov_0_3 -> bcb3__Cov_0_3 [label=<false>];
+     bcb1__Cov_0_3 -> bcb2__Cov_0_3 [label=<otherwise>];
+     bcb0__Cov_0_3 -> bcb1__Cov_0_3 [label=<>];
12 


thread '[mir-opt] mir-opt/coverage_graphviz.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/coverage_graphviz.main.InstrumentCoverage.0.dot', src/tools/compiletest/src/runtest.rs:3588:25

---- [mir-opt] mir-opt/instrument_coverage.rs stdout ----
---- [mir-opt] mir-opt/instrument_coverage.rs stdout ----
8       let mut _3: !;                       // in scope 0 at /the/src/instrument_coverage.rs:12:18: 14:10
10       bb0: {
10       bb0: {
- +         Coverage::Counter(1) for /the/src/instrument_coverage.rs:10:1 - 12:17; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
-           falseUnwind -> [real: bb1, cleanup: bb5]; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+ +         Coverage::Counter(1) for /the/src/instrument_coverage.rs:10:1 - 10:11; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+           goto -> bb1;                     // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
14   
15       bb1: {


+ +         Coverage::Expression(4294967295) = 1 + 2 for /the/src/instrument_coverage.rs:11:5 - 12:17; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+           falseUnwind -> [real: bb2, cleanup: bb6]; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+   
+       bb2: {
+       bb2: {
16           StorageLive(_2);                 // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
-           _2 = bar() -> [return: bb2, unwind: bb5]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
+           _2 = bar() -> [return: bb3, unwind: bb6]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
18                                            // mir::Constant
19                                            // + span: /the/src/instrument_coverage.rs:12:12: 12:15
20                                            // + literal: Const { ty: fn() -> bool {bar}, val: Value(Scalar(<ZST>)) }
21       }
22   
-       bb2: {
-       bb2: {
-           switchInt(move _2) -> [false: bb4, otherwise: bb3]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
+       bb3: {
+           switchInt(move _2) -> [false: bb5, otherwise: bb4]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
26   
-       bb3: {
-       bb3: {
- +         Coverage::Expression(4294967294) = 2 + 0 for /the/src/instrument_coverage.rs:16:1 - 16:2; // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
- +         Coverage::Counter(2) for /the/src/instrument_coverage.rs:13:13 - 13:18; // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
+       bb4: {
+ +         Coverage::Expression(4294967293) = 4294967294 + 0 for /the/src/instrument_coverage.rs:16:1 - 16:2; // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
+ +         Coverage::Expression(4294967294) = 4294967295 - 2 for /the/src/instrument_coverage.rs:13:13 - 13:18; // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
30           _0 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:13:13: 13:18
31           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:14:9: 14:10
32           return;                          // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
33       }
34   
-       bb4: {
-       bb4: {
- +         Coverage::Expression(4294967295) = 1 - 2 for /the/src/instrument_coverage.rs:14:10 - 14:11; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+       bb5: {
+ +         Coverage::Counter(2) for /the/src/instrument_coverage.rs:14:10 - 14:11; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
37           _1 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:14:10: 14:10
38           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:14:9: 14:10
-           goto -> bb0;                     // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+           goto -> bb1;                     // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
41   
41   
-       bb5 (cleanup): {
+       bb6 (cleanup): {
43           resume;                          // scope 0 at /the/src/instrument_coverage.rs:10:1: 16:2
45   }


thread '[mir-opt] mir-opt/instrument_coverage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/instrument_coverage.main.InstrumentCoverage.diff', src/tools/compiletest/src/runtest.rs:3588:25

failures:
    [mir-opt] mir-opt/coverage_graphviz.rs
    [mir-opt] mir-opt/instrument_coverage.rs
    [mir-opt] mir-opt/instrument_coverage.rs

test result: FAILED. 160 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out; finished in 2.26s



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:19:57
