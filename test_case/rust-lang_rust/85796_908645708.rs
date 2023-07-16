plain

---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
23       }
24   
25       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/optimizes_into_variable.rs:12:9: 12:10
+           nop;                             // scope 0 at $DIR/optimizes_into_variable.rs:12:9: 12:10
27 -         _2 = CheckedAdd(const 2_i32, const 2_i32); // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
28 -         assert(!move (_2.1: bool), "attempt to compute `{} + {}`, which would overflow", const 2_i32, const 2_i32) -> bb1; // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
29 +         _2 = const (4_i32, false);       // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
33       bb1: {
33       bb1: {
34 -         _1 = move (_2.0: i32);           // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
35 +         _1 = const 4_i32;                // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
-           StorageLive(_3);                 // scope 1 at $DIR/optimizes_into_variable.rs:13:9: 13:10
-           StorageLive(_4);                 // scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:31
+           nop;                             // scope 1 at $DIR/optimizes_into_variable.rs:13:9: 13:10
+           nop;                             // scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:31
38           _4 = [const 0_i32, const 1_i32, const 2_i32, const 3_i32, const 4_i32, const 5_i32]; // scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:31
-           StorageLive(_5);                 // scope 1 at $DIR/optimizes_into_variable.rs:13:32: 13:33
+           nop;                             // scope 1 at $DIR/optimizes_into_variable.rs:13:32: 13:33
40           _5 = const 3_usize;              // scope 1 at $DIR/optimizes_into_variable.rs:13:32: 13:33
41           _6 = const 6_usize;              // scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:34
42 -         _7 = Lt(_5, _6);                 // scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:34
48       bb2: {
48       bb2: {
49 -         _3 = _4[_5];                     // scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:34
50 +         _3 = const 3_i32;                // scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:34
-           StorageDead(_5);                 // scope 1 at $DIR/optimizes_into_variable.rs:13:34: 13:35
-           StorageDead(_4);                 // scope 1 at $DIR/optimizes_into_variable.rs:13:34: 13:35
-           StorageLive(_8);                 // scope 2 at $DIR/optimizes_into_variable.rs:14:9: 14:10
-           nop;                             // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:36
+           nop;                             // scope 1 at $DIR/optimizes_into_variable.rs:13:34: 13:35
+           nop;                             // scope 1 at $DIR/optimizes_into_variable.rs:13:34: 13:35
+           nop;                             // scope 2 at $DIR/optimizes_into_variable.rs:14:9: 14:10
+           StorageLive(_9);                 // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:36
55           _9 = const 42_u32;               // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:36
56 -         _8 = _9;                         // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:38
57 +         _8 = const 42_u32;               // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:38

-           nop;                             // scope 2 at $DIR/optimizes_into_variable.rs:14:38: 14:39
+           StorageDead(_9);                 // scope 2 at $DIR/optimizes_into_variable.rs:14:38: 14:39
59           nop;                             // scope 0 at $DIR/optimizes_into_variable.rs:11:11: 15:2
-           StorageDead(_8);                 // scope 2 at $DIR/optimizes_into_variable.rs:15:1: 15:2
-           StorageDead(_3);                 // scope 1 at $DIR/optimizes_into_variable.rs:15:1: 15:2
-           StorageDead(_1);                 // scope 0 at $DIR/optimizes_into_variable.rs:15:1: 15:2
+           nop;                             // scope 2 at $DIR/optimizes_into_variable.rs:15:1: 15:2
+           nop;                             // scope 1 at $DIR/optimizes_into_variable.rs:15:1: 15:2
+           nop;                             // scope 0 at $DIR/optimizes_into_variable.rs:15:1: 15:2
63           return;                          // scope 0 at $DIR/optimizes_into_variable.rs:15:2: 15:2
65   }


thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/optimizes_into_variable.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
45       }
46   
46   
47       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/issue-73223.rs:2:9: 2:14
-           StorageLive(_2);                 // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
+           nop;                             // scope 0 at $DIR/issue-73223.rs:2:9: 2:14
+           nop;                             // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
50           ((_2 as Some).0: i32) = const 1_i32; // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
51           discriminant(_2) = 1;            // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
52           _3 = const 1_isize;              // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
55   
56       bb1: {
56       bb1: {
57           nop;                             // scope 0 at $DIR/issue-73223.rs:4:17: 4:23
-           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:5:6: 5:7
-           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
+           nop;                             // scope 0 at $DIR/issue-73223.rs:5:6: 5:7
+           nop;                             // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
60           return;                          // scope 0 at $DIR/issue-73223.rs:9:2: 9:2
62   

63       bb2: {
63       bb2: {
-           StorageLive(_4);                 // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
+           nop;                             // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
65           _4 = ((_2 as Some).0: i32);      // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
66           _1 = _4;                         // scope 2 at $DIR/issue-73223.rs:3:20: 3:21
-           StorageDead(_4);                 // scope 0 at $DIR/issue-73223.rs:3:20: 3:21
-           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:5:6: 5:7
-           StorageLive(_5);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
-           StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+           nop;                             // scope 0 at $DIR/issue-73223.rs:3:20: 3:21
+           nop;                             // scope 0 at $DIR/issue-73223.rs:5:6: 5:7
+           nop;                             // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
+           nop;                             // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
71           _6 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
72           ((_5 as Some).0: i32) = move _6; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
73           discriminant(_5) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28

-           StorageDead(_6);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
+           nop;                             // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
75           nop;                             // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_24);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_23);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
76           StorageLive(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
77           _7 = &_1;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
78           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

143           StorageDead(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
144           StorageDead(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
145           StorageDead(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_24);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_23);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
146           nop;                             // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
147           nop;                             // scope 0 at $DIR/issue-73223.rs:1:11: 9:2
-           StorageDead(_5);                 // scope 1 at $DIR/issue-73223.rs:9:1: 9:2
-           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
+           nop;                             // scope 1 at $DIR/issue-73223.rs:9:1: 9:2
+           nop;                             // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
150           return;                          // scope 0 at $DIR/issue-73223.rs:9:2: 9:2
152   }


thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25

failures:
    [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs
    [mir-opt] mir-opt/issue-73223.rs
    [mir-opt] mir-opt/issue-73223.rs

test result: FAILED. 160 passed; 2 failed; 4 ignored; 0 measured; 0 filtered out; finished in 3.69s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:37
