plain
test [mir-opt] mir-opt/storage_live_dead_in_statics.rs ... ok

failures:

---- [mir-opt] mir-opt/coverage_graphviz.rs stdout ----
3     node [fontname="Courier, monospace"];
4     edge [fontname="Courier, monospace"];
5     bcb2__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb2</td></tr><tr><td align="left" balign="left">Expression(bcb0 - bcb1) at 13:10-13:10<br/>    13:10-13:10: @4[0]: Coverage::Expression(4294967295) = 1 - 2 for $DIR/coverage_graphviz.rs:13:10 - 13:11</td></tr><tr><td align="left" balign="left">bb4: Goto</td></tr></table>>];
-     bcb1__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb1</td></tr><tr><td align="left" balign="left">Counter(bcb1) at 12:13-12:18<br/>    12:13-12:18: @5[0]: _0 = const ()<br/>Expression(bcb1 + 0) at 15:2-15:2<br/>    15:2-15:2: @5.Return: return</td></tr><tr><td align="left" balign="left">bb3: FalseEdge</td></tr><tr><td align="left" balign="left">bb5: Return</td></tr></table>>];
-     bcb0__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb0</td></tr><tr><td align="left" balign="left"></td></tr><tr><td align="left" balign="left">Counter(bcb0) at 9:1-11:17<br/>    11:12-11:17: @1.Call: _2 = bar() -&gt; [return: bb2, unwind: bb6]<br/>    11:12-11:17: @2[0]: FakeRead(ForMatchedPlace, _2)</td></tr><tr><td align="left" balign="left">bb0: FalseUnwind<br/>bb1: Call</td></tr><tr><td align="left" balign="left">bb2: SwitchInt</td></tr></table>>];
+     bcb1__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb1</td></tr><tr><td align="left" balign="left">Counter(bcb1) at 12:13-12:18<br/>    12:13-12:18: @3[0]: Coverage::Expression(4294967294) = 2 + 0 for $DIR/coverage_graphviz.rs:15:1 - 15:2<br/>Expression(bcb1 + 0) at 15:2-15:2<br/>    15:2-15:2: @3.Return: return</td></tr><tr><td align="left" balign="left">bb3: Return</td></tr></table>>];
+     bcb0__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb0</td></tr><tr><td align="left" balign="left"></td></tr><tr><td align="left" balign="left">Counter(bcb0) at 9:1-11:17<br/>    11:12-11:17: @1.Call: _2 = bar() -&gt; [return: bb2, unwind: bb5]</td></tr><tr><td align="left" balign="left">bb0: FalseUnwind<br/>bb1: Call</td></tr><tr><td align="left" balign="left">bb2: SwitchInt</td></tr></table>>];
8     bcb2__Cov_0_3 -> bcb0__Cov_0_3 [label=<>];
9     bcb0__Cov_0_3 -> bcb2__Cov_0_3 [label=<false>];
10     bcb0__Cov_0_3 -> bcb1__Cov_0_3 [label=<otherwise>];

thread '[mir-opt] mir-opt/coverage_graphviz.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/coverage_graphviz.main.InstrumentCoverage.0.dot', src/tools/compiletest/src/runtest.rs:3452:25

---- [mir-opt] mir-opt/instrument_coverage.rs stdout ----
9   
10       bb0: {
10       bb0: {
11 +         Coverage::Counter(1) for /the/src/instrument_coverage.rs:10:1 - 12:17; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
-           falseUnwind -> [real: bb1, cleanup: bb6]; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+           falseUnwind -> [real: bb1, cleanup: bb5]; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
14   
15       bb1: {


16           StorageLive(_2);                 // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
-           _2 = bar() -> [return: bb2, unwind: bb6]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
+           _2 = bar() -> [return: bb2, unwind: bb5]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
18                                            // mir::Constant
19                                            // + span: /the/src/instrument_coverage.rs:12:12: 12:15
20                                            // + literal: Const { ty: fn() -> bool {bar}, val: Value(Scalar(<ZST>)) }
21       }
22   
23       bb2: {
23       bb2: {
-           FakeRead(ForMatchedPlace, _2);   // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
-           switchInt(_2) -> [false: bb4, otherwise: bb3]; // scope 0 at /the/src/instrument_coverage.rs:12:9: 14:10
+           switchInt(move _2) -> [false: bb4, otherwise: bb3]; // scope 0 at /the/src/instrument_coverage.rs:12:9: 14:10
27   
28       bb3: {


- +         Coverage::Expression(4294967294) = 2 + 0 for /the/src/instrument_coverage.rs:16:1 - 16:2; // scope 0 at /the/src/instrument_coverage.rs:12:9: 14:10
- +         Coverage::Counter(2) for /the/src/instrument_coverage.rs:13:13 - 13:18; // scope 0 at /the/src/instrument_coverage.rs:12:9: 14:10
-           falseEdge -> [real: bb5, imaginary: bb4]; // scope 0 at /the/src/instrument_coverage.rs:12:9: 14:10
+ +         Coverage::Expression(4294967294) = 2 + 0 for /the/src/instrument_coverage.rs:16:1 - 16:2; // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
+ +         Coverage::Counter(2) for /the/src/instrument_coverage.rs:13:13 - 13:18; // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
+           _0 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:13:13: 13:18
+           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:14:9: 14:10
+           return;                          // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
33   
34       bb4: {


35 +         Coverage::Expression(4294967295) = 1 - 2 for /the/src/instrument_coverage.rs:14:10 - 14:11; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
36           _1 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:14:10: 14:10
-           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:15:5: 15:6
+           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:14:9: 14:10
38           goto -> bb0;                     // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
40   

-       bb5: {
-       bb5: {
-           _0 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:13:13: 13:18
-           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:15:5: 15:6
-           return;                          // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
-   
-   
-       bb6 (cleanup): {
+       bb5 (cleanup): {
48           resume;                          // scope 0 at /the/src/instrument_coverage.rs:10:1: 16:2
50   }


thread '[mir-opt] mir-opt/instrument_coverage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/instrument_coverage.main.InstrumentCoverage.diff', src/tools/compiletest/src/runtest.rs:3452:25

failures:
    [mir-opt] mir-opt/coverage_graphviz.rs
    [mir-opt] mir-opt/instrument_coverage.rs
    [mir-opt] mir-opt/instrument_coverage.rs

test result: FAILED. 148 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out; finished in 5.03s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.0-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:23:49
