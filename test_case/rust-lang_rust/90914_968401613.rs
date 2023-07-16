plain
................................i........F........................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
15       scope 1 {
16           debug residual => _6;            // in scope 1 at $DIR/separate_const_switch.rs:29:9: 29:10
17           scope 2 {
-               scope 8 (inlined <Result<i32, i32> as FromResidual<Result<Infallible, i32>>>::from_residual) { // at $DIR/separate_const_switch.rs:29:8: 29:10
-                   debug residual => _8;    // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-                   let _16: i32;            // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-                   let mut _17: i32;        // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-                   let mut _18: i32;        // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-                   scope 9 {
-                       debug e => _16;      // in scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-                       scope 10 (inlined <i32 as From<i32>>::from) { // at $DIR/separate_const_switch.rs:29:8: 29:10
-                           debug t => _18;  // in scope 10 at $DIR/separate_const_switch.rs:29:8: 29:10
+               scope 5 (inlined <Result<i32, i32> as FromResidual<Result<Infallible, i32>>>::from_residual) { // at $DIR/separate_const_switch.rs:29:8: 29:10
+                   debug residual => _8;    // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
+                   let _10: i32;            // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
+                   let mut _11: i32;        // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
+                   let mut _12: i32;        // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
+                   scope 6 {
+                       debug e => _10;      // in scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
+                       scope 7 (inlined <i32 as From<i32>>::from) { // at $DIR/separate_const_switch.rs:29:8: 29:10
+                           debug t => _12;  // in scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
28                   }
29               }

34           scope 4 {
34           scope 4 {
35           }
36       }
-       scope 5 (inlined <Result<i32, i32> as Try>::branch) { // at $DIR/separate_const_switch.rs:29:8: 29:10
-           debug self => _4;                // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           let mut _10: isize;              // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           let _11: i32;                    // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           let mut _12: i32;                // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           let _13: i32;                    // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           let mut _14: std::result::Result<std::convert::Infallible, i32>; // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           let mut _15: i32;                // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           scope 6 {
-               debug v => _11;              // in scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
-           scope 7 {
-           scope 7 {
-               debug e => _13;              // in scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-       }
52   
53       bb0: {
53       bb0: {
54           StorageLive(_2);                 // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10

55           StorageLive(_3);                 // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
56           StorageLive(_4);                 // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:9
57           _4 = _1;                         // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:9
-           StorageLive(_10);                // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _10 = discriminant(_4);          // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
- -         switchInt(move _10) -> [0_isize: bb6, 1_isize: bb4, otherwise: bb5]; // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
- +         switchInt(move _10) -> [0_isize: bb5, 1_isize: bb3, otherwise: bb4]; // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
+           _3 = <Result<i32, i32> as Try>::branch(move _4) -> bb1; // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
+                                            // mir::Constant
+                                            // + span: $DIR/separate_const_switch.rs:29:8: 29:10
+                                            // + literal: Const { ty: fn(std::result::Result<i32, i32>) -> std::ops::ControlFlow<<std::result::Result<i32, i32> as std::ops::Try>::Residual, <std::result::Result<i32, i32> as std::ops::Try>::Output> {<std::result::Result<i32, i32> as std::ops::Try>::branch}, val: Value(Scalar(<ZST>)) }
63   
64       bb1: {


- -         StorageDead(_10);                // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
- -         StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
- -         _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
- -         switchInt(move _5) -> [0_isize: bb2, otherwise: bb3]; // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
- -     }
- -     bb2: {
- -     bb2: {
+           StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
+           _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
+           switchInt(move _5) -> [0_isize: bb2, otherwise: bb3]; // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
+   
+       bb2: {
+       bb2: {
72           StorageLive(_9);                 // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
73           _9 = ((_3 as Continue).0: i32);  // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
74           _2 = _9;                         // scope 4 at $DIR/separate_const_switch.rs:29:8: 29:10

80           return;                          // scope 0 at $DIR/separate_const_switch.rs:30:2: 30:2
82   
- -     bb3: {
- +     bb2: {
+       bb3: {
+       bb3: {
85           StorageLive(_6);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
86           _6 = ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>); // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
87           StorageLive(_8);                 // scope 2 at $DIR/separate_const_switch.rs:29:9: 29:10

88           _8 = _6;                         // scope 2 at $DIR/separate_const_switch.rs:29:9: 29:10
-           StorageLive(_16);                // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _16 = move ((_8 as Err).0: i32); // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageLive(_17);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageLive(_18);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _18 = move _16;                  // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _17 = move _18;                  // scope 10 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_18);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           ((_0 as Err).0: i32) = move _17; // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           discriminant(_0) = 1;            // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_17);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_16);                // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageLive(_10);                // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
+           _10 = move ((_8 as Err).0: i32); // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageLive(_11);                // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageLive(_12);                // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
+           _12 = move _10;                  // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
+           _11 = move _12;                  // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageDead(_12);                // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
+           ((_0 as Err).0: i32) = move _11; // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
+           discriminant(_0) = 1;            // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageDead(_11);                // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageDead(_10);                // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
100           StorageDead(_8);                 // scope 2 at $DIR/separate_const_switch.rs:29:9: 29:10
101           StorageDead(_6);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
102           StorageDead(_2);                 // scope 0 at $DIR/separate_const_switch.rs:29:10: 29:11

103           StorageDead(_3);                 // scope 0 at $DIR/separate_const_switch.rs:30:1: 30:2
104           return;                          // scope 0 at $DIR/separate_const_switch.rs:30:2: 30:2
-   
- -     bb4: {
- +     bb3: {
- +     bb3: {
-           StorageLive(_13);                // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _13 = move ((_4 as Err).0: i32); // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageLive(_14);                // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageLive(_15);                // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _15 = move _13;                  // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           ((_14 as Err).0: i32) = move _15; // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           discriminant(_14) = 1;           // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_15);                // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>) = move _14; // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           discriminant(_3) = 1;            // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_14);                // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_13);                // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
- -         goto -> bb1;                     // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
- +         StorageDead(_10);                // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
- +         StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
- +         _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
- +         switchInt(move _5) -> [0_isize: bb1, otherwise: bb2]; // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
-   
- -     bb5: {
- +     bb4: {
- +     bb4: {
-           unreachable;                     // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-   
- -     bb6: {
- +     bb5: {
- +     bb5: {
-           StorageLive(_11);                // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _11 = move ((_4 as Ok).0: i32);  // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageLive(_12);                // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _12 = move _11;                  // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
-           ((_3 as Continue).0: i32) = move _12; // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
-           discriminant(_3) = 0;            // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_12);                // scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_11);                // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
- -         goto -> bb1;                     // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
- +         StorageDead(_10);                // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
- +         StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
- +         _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
- +         switchInt(move _5) -> [0_isize: bb1, otherwise: bb2]; // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
149   }
150   


thread '[mir-opt] mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/separate_const_switch.identity.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3357:25


failures:
    [mir-opt] mir-opt/separate_const_switch.rs
    [mir-opt] mir-opt/separate_const_switch.rs

test result: FAILED. 162 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 3.24s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:24
