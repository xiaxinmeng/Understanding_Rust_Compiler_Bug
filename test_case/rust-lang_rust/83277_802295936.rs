plain
.................................................................................................... 9300/11700
.................................................................................................... 9400/11700
.................................................................................................... 9500/11700
...........................................i......i................................................. 9600/11700
..........................................................................................iiiiiii.ii 9700/11700
.................................................................................................... 9900/11700
.................................................................................................... 10000/11700
.................................................................................................... 10100/11700
.................................................................................................... 10200/11700
---
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..........................i................................
failures:

---- [mir-opt] mir-opt/early_otherwise_branch_3_element_tuple.rs stdout ----
16       let _11: u32;                        // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:15: 6:16
17       let _12: u32;                        // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:24: 6:25
18       let _13: u32;                        // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:33: 6:34
- +     let mut _14: isize;                  // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
- +     let mut _15: bool;                   // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
- +     let mut _16: isize;                  // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:28: 6:35
- +     let mut _17: bool;                   // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:28: 6:35
23       scope 1 {
24           debug a => _11;                  // in scope 1 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:15: 6:16
25           debug b => _12;                  // in scope 1 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:24: 6:25

41           StorageDead(_6);                 // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:19: 5:20
42           StorageDead(_5);                 // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:19: 5:20
43           _10 = discriminant((_4.0: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:10: 6:17
- -         switchInt(move _10) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:10: 6:17
- +         StorageLive(_14);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:10: 6:17
- +         _14 = discriminant((_4.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:10: 6:17
- +         StorageLive(_15);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:10: 6:17
- +         _15 = Ne(_14, _10);              // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:10: 6:17
- +         StorageDead(_14);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:10: 6:17
- +         switchInt(move _15) -> [false: bb5, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:10: 6:17
+           switchInt(move _10) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:10: 6:17
52   
53       bb1: {


- +         StorageDead(_17);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:7:14: 7:15
- +         StorageDead(_15);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:7:14: 7:15
56           _0 = const 1_u32;                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:7:14: 7:15
- -         goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 8:6
- +         goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 8:6
+           goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 8:6
60   
61       bb2: {


- -         _9 = discriminant((_4.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
- -         switchInt(move _9) -> [1_isize: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
- -     }
- -     bb3: {
- -     bb3: {
+           _9 = discriminant((_4.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
+           switchInt(move _9) -> [1_isize: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
+   
+       bb3: {
+       bb3: {
67           _8 = discriminant((_4.2: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:28: 6:35
- -         switchInt(move _8) -> [1_isize: bb4, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:28: 6:35
- +         switchInt(move _8) -> [1_isize: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:28: 6:35
+           switchInt(move _8) -> [1_isize: bb4, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:28: 6:35
71   
- -     bb4: {
- +     bb3: {
+       bb4: {
+       bb4: {
74           StorageLive(_11);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:15: 6:16
75           _11 = (((_4.0: std::option::Option<u32>) as Some).0: u32); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:15: 6:16
76           StorageLive(_12);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:24: 6:25

81           StorageDead(_13);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:40: 6:41
82           StorageDead(_12);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:40: 6:41
83           StorageDead(_11);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:40: 6:41
- -         goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 8:6
- +         goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 8:6
+           goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 8:6
87   
- -     bb5: {
- +     bb4: {
+       bb5: {
+       bb5: {
90           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:9:1: 9:2
91           return;                          // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:9:2: 9:2
- +     }
- +     bb5: {
- +     bb5: {
- +         StorageDead(_15);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
- +         switchInt(_10) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
98   }
99   


thread '[mir-opt] mir-opt/early_otherwise_branch_3_element_tuple.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch_3_element_tuple.opt1.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3517:25

---- [mir-opt] mir-opt/early_otherwise_branch.rs stdout ----
---- [mir-opt] mir-opt/early_otherwise_branch.rs stdout ----
12       let mut _7: isize;                   // in scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
13       let _8: u32;                         // in scope 0 at $DIR/early_otherwise_branch.rs:5:15: 5:16
14       let _9: u32;                         // in scope 0 at $DIR/early_otherwise_branch.rs:5:24: 5:25
- +     let mut _10: isize;                  // in scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
- +     let mut _11: bool;                   // in scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
17       scope 1 {
18           debug a => _8;                   // in scope 1 at $DIR/early_otherwise_branch.rs:5:15: 5:16
19           debug b => _9;                   // in scope 1 at $DIR/early_otherwise_branch.rs:5:24: 5:25

30           StorageDead(_5);                 // scope 0 at $DIR/early_otherwise_branch.rs:4:16: 4:17
31           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch.rs:4:16: 4:17
32           _7 = discriminant((_3.0: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
- -         switchInt(move _7) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
- +         StorageLive(_10);                // scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
- +         _10 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
- +         StorageLive(_11);                // scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
- +         _11 = Ne(_10, _7);               // scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
- +         StorageDead(_10);                // scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
- +         switchInt(move _11) -> [false: bb4, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
+           switchInt(move _7) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
41   
42       bb1: {


- +         StorageDead(_11);                // scope 0 at $DIR/early_otherwise_branch.rs:6:14: 6:15
44           _0 = const 1_u32;                // scope 0 at $DIR/early_otherwise_branch.rs:6:14: 6:15
- -         goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 7:6
- +         goto -> bb3;                     // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 7:6
+           goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 7:6
48   
49       bb2: {


- -         _6 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
- -         switchInt(move _6) -> [1_isize: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
- -     }
- -     bb3: {
- -     bb3: {
+           _6 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
+           switchInt(move _6) -> [1_isize: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
+   
+       bb3: {
+       bb3: {
55           StorageLive(_8);                 // scope 0 at $DIR/early_otherwise_branch.rs:5:15: 5:16
56           _8 = (((_3.0: std::option::Option<u32>) as Some).0: u32); // scope 0 at $DIR/early_otherwise_branch.rs:5:15: 5:16
57           StorageLive(_9);                 // scope 0 at $DIR/early_otherwise_branch.rs:5:24: 5:25

59           _0 = const 0_u32;                // scope 1 at $DIR/early_otherwise_branch.rs:5:31: 5:32
60           StorageDead(_9);                 // scope 0 at $DIR/early_otherwise_branch.rs:5:31: 5:32
61           StorageDead(_8);                 // scope 0 at $DIR/early_otherwise_branch.rs:5:31: 5:32
- -         goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 7:6
- +         goto -> bb3;                     // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 7:6
+           goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 7:6
65   
- -     bb4: {
- +     bb3: {
+       bb4: {
+       bb4: {
68           StorageDead(_3);                 // scope 0 at $DIR/early_otherwise_branch.rs:8:1: 8:2
69           return;                          // scope 0 at $DIR/early_otherwise_branch.rs:8:2: 8:2
- +     }
- +     bb4: {
- +     bb4: {
- +         StorageDead(_11);                // scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
- +         switchInt(_7) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
76   }
77   


thread '[mir-opt] mir-opt/early_otherwise_branch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch.opt1.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3517:25

failures:
    [mir-opt] mir-opt/early_otherwise_branch.rs
    [mir-opt] mir-opt/early_otherwise_branch_3_element_tuple.rs
    [mir-opt] mir-opt/early_otherwise_branch_3_element_tuple.rs

test result: FAILED. 154 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.96s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:42
