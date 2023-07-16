plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
........F..................i................................
failures:

---- [mir-opt] mir-opt/issues/issue-75439.rs stdout ----
2 + // MIR for `foo` after MatchBranchSimplification
3   
4   fn foo(_1: [u8; 16]) -> Option<[u8; 4]> {
-       debug bytes => _1;                   // in scope 0 at $DIR/issue-75439.rs:7:12: 7:17
-       let mut _0: std::option::Option<[u8; 4]>; // return place in scope 0 at $DIR/issue-75439.rs:7:32: 7:47
-       let _2: [u32; 4];                    // in scope 0 at $DIR/issue-75439.rs:9:9: 9:15
-       let mut _3: [u8; 16];                // in scope 0 at $DIR/issue-75439.rs:9:47: 9:52
-       let mut _5: [u8; 4];                 // in scope 0 at $DIR/issue-75439.rs:12:14: 12:38
-       let mut _6: u32;                     // in scope 0 at $DIR/issue-75439.rs:12:33: 12:35
+       debug bytes => _1;                   // in scope 0 at $DIR/issue-75439.rs:5:12: 5:17
+       let mut _0: std::option::Option<[u8; 4]>; // return place in scope 0 at $DIR/issue-75439.rs:5:32: 5:47
+       let _2: [u32; 4];                    // in scope 0 at $DIR/issue-75439.rs:7:9: 7:15
+       let mut _3: [u8; 16];                // in scope 0 at $DIR/issue-75439.rs:7:47: 7:52
+       let mut _5: [u8; 4];                 // in scope 0 at $DIR/issue-75439.rs:10:14: 10:38
+       let mut _6: u32;                     // in scope 0 at $DIR/issue-75439.rs:10:33: 10:35
11       scope 1 {
-           debug dwords => _2;              // in scope 1 at $DIR/issue-75439.rs:9:9: 9:15
-           let _4: u32;                     // in scope 1 at $DIR/issue-75439.rs:11:27: 11:29
+           debug dwords => _2;              // in scope 1 at $DIR/issue-75439.rs:7:9: 7:15
+           let _4: u32;                     // in scope 1 at $DIR/issue-75439.rs:9:27: 9:29
14           scope 3 {
-               debug ip => _4;              // in scope 3 at $DIR/issue-75439.rs:11:27: 11:29
+               debug ip => _4;              // in scope 3 at $DIR/issue-75439.rs:9:27: 9:29
16               scope 4 {
18           }

21       }
22   
22   
23       bb0: {
-           StorageLive(_2);                 // scope 0 at $DIR/issue-75439.rs:9:9: 9:15
-           StorageLive(_3);                 // scope 2 at $DIR/issue-75439.rs:9:47: 9:52
-           _3 = _1;                         // scope 2 at $DIR/issue-75439.rs:9:47: 9:52
-           _2 = transmute::<[u8; 16], [u32; 4]>(move _3) -> bb1; // scope 2 at $DIR/issue-75439.rs:9:37: 9:53
+           StorageLive(_2);                 // scope 0 at $DIR/issue-75439.rs:7:9: 7:15
+           StorageLive(_3);                 // scope 2 at $DIR/issue-75439.rs:7:47: 7:52
+           _3 = _1;                         // scope 2 at $DIR/issue-75439.rs:7:47: 7:52
+           _2 = transmute::<[u8; 16], [u32; 4]>(move _3) -> bb1; // scope 2 at $DIR/issue-75439.rs:7:37: 7:53
28                                            // mir::Constant
-                                            // + span: $DIR/issue-75439.rs:9:37: 9:46
+                                            // + span: $DIR/issue-75439.rs:7:37: 7:46
30                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn([u8; 16]) -> [u32; 4] {std::intrinsics::transmute::<[u8; 16], [u32; 4]>}, val: Value(Scalar(<ZST>)) }
32   

33       bb1: {
33       bb1: {
-           StorageDead(_3);                 // scope 2 at $DIR/issue-75439.rs:9:52: 9:53
-           switchInt(_2[0 of 4]) -> [0_u32: bb2, otherwise: bb4]; // scope 1 at $DIR/issue-75439.rs:11:13: 11:14
+           StorageDead(_3);                 // scope 2 at $DIR/issue-75439.rs:7:52: 7:53
+           switchInt(_2[0 of 4]) -> [0_u32: bb2, otherwise: bb4]; // scope 1 at $DIR/issue-75439.rs:9:13: 9:14
37   
38       bb2: {


-           switchInt(_2[1 of 4]) -> [0_u32: bb3, otherwise: bb4]; // scope 1 at $DIR/issue-75439.rs:11:16: 11:17
+           switchInt(_2[1 of 4]) -> [0_u32: bb3, otherwise: bb4]; // scope 1 at $DIR/issue-75439.rs:9:16: 9:17
41   
42       bb3: {


-           switchInt(_2[2 of 4]) -> [0_u32: bb6, 4294901760_u32: bb7, otherwise: bb4]; // scope 1 at $DIR/issue-75439.rs:11:19: 11:20
+           switchInt(_2[2 of 4]) -> [0_u32: bb6, 4294901760_u32: bb7, otherwise: bb4]; // scope 1 at $DIR/issue-75439.rs:9:19: 9:20
45   
46       bb4: {


-           discriminant(_0) = 0;            // scope 1 at $DIR/issue-75439.rs:14:9: 14:13
-           goto -> bb9;                     // scope 1 at $DIR/issue-75439.rs:11:5: 15:6
+           discriminant(_0) = 0;            // scope 1 at $DIR/issue-75439.rs:12:9: 12:13
+           goto -> bb9;                     // scope 1 at $DIR/issue-75439.rs:9:5: 13:6
50   
51       bb5: {


-           StorageLive(_5);                 // scope 3 at $DIR/issue-75439.rs:12:14: 12:38
-           StorageLive(_6);                 // scope 4 at $DIR/issue-75439.rs:12:33: 12:35
-           _6 = _4;                         // scope 4 at $DIR/issue-75439.rs:12:33: 12:35
-           _5 = transmute::<u32, [u8; 4]>(move _6) -> bb8; // scope 4 at $DIR/issue-75439.rs:12:23: 12:36
+           StorageLive(_5);                 // scope 3 at $DIR/issue-75439.rs:10:14: 10:38
+           StorageLive(_6);                 // scope 4 at $DIR/issue-75439.rs:10:33: 10:35
+           _6 = _4;                         // scope 4 at $DIR/issue-75439.rs:10:33: 10:35
+           _5 = transmute::<u32, [u8; 4]>(move _6) -> bb8; // scope 4 at $DIR/issue-75439.rs:10:23: 10:36
56                                            // mir::Constant
-                                            // + span: $DIR/issue-75439.rs:12:23: 12:32
+                                            // + span: $DIR/issue-75439.rs:10:23: 10:32
58                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(u32) -> [u8; 4] {std::intrinsics::transmute::<u32, [u8; 4]>}, val: Value(Scalar(<ZST>)) }
60   

61       bb6: {
61       bb6: {
-           StorageLive(_4);                 // scope 1 at $DIR/issue-75439.rs:11:27: 11:29
-           _4 = _2[3 of 4];                 // scope 1 at $DIR/issue-75439.rs:11:27: 11:29
-           goto -> bb5;                     // scope 1 at $DIR/issue-75439.rs:11:5: 15:6
+           StorageLive(_4);                 // scope 1 at $DIR/issue-75439.rs:9:27: 9:29
+           _4 = _2[3 of 4];                 // scope 1 at $DIR/issue-75439.rs:9:27: 9:29
+           goto -> bb5;                     // scope 1 at $DIR/issue-75439.rs:9:5: 13:6
66   
67       bb7: {


-           StorageLive(_4);                 // scope 1 at $DIR/issue-75439.rs:11:27: 11:29
-           _4 = _2[3 of 4];                 // scope 1 at $DIR/issue-75439.rs:11:27: 11:29
-           goto -> bb5;                     // scope 1 at $DIR/issue-75439.rs:11:5: 15:6
+           StorageLive(_4);                 // scope 1 at $DIR/issue-75439.rs:9:27: 9:29
+           _4 = _2[3 of 4];                 // scope 1 at $DIR/issue-75439.rs:9:27: 9:29
+           goto -> bb5;                     // scope 1 at $DIR/issue-75439.rs:9:5: 13:6
72   
73       bb8: {


-           StorageDead(_6);                 // scope 4 at $DIR/issue-75439.rs:12:35: 12:36
-           ((_0 as Some).0: [u8; 4]) = move _5; // scope 3 at $DIR/issue-75439.rs:12:9: 12:39
-           discriminant(_0) = 1;            // scope 3 at $DIR/issue-75439.rs:12:9: 12:39
-           StorageDead(_5);                 // scope 3 at $DIR/issue-75439.rs:12:38: 12:39
-           StorageDead(_4);                 // scope 1 at $DIR/issue-75439.rs:13:5: 13:6
-           goto -> bb9;                     // scope 1 at $DIR/issue-75439.rs:11:5: 15:6
+           StorageDead(_6);                 // scope 4 at $DIR/issue-75439.rs:10:35: 10:36
+           ((_0 as Some).0: [u8; 4]) = move _5; // scope 3 at $DIR/issue-75439.rs:10:9: 10:39
+           discriminant(_0) = 1;            // scope 3 at $DIR/issue-75439.rs:10:9: 10:39
+           StorageDead(_5);                 // scope 3 at $DIR/issue-75439.rs:10:38: 10:39
+           StorageDead(_4);                 // scope 1 at $DIR/issue-75439.rs:11:5: 11:6
+           goto -> bb9;                     // scope 1 at $DIR/issue-75439.rs:9:5: 13:6
81   
82       bb9: {


-           StorageDead(_2);                 // scope 0 at $DIR/issue-75439.rs:16:1: 16:2
-           return;                          // scope 0 at $DIR/issue-75439.rs:16:2: 16:2
+           StorageDead(_2);                 // scope 0 at $DIR/issue-75439.rs:14:1: 14:2
+           return;                          // scope 0 at $DIR/issue-75439.rs:14:2: 14:2
86   }
87   


thread '[mir-opt] mir-opt/issues/issue-75439.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issues/issue_75439.foo.MatchBranchSimplification.diff', src/tools/compiletest/src/runtest.rs:3558:25


failures:
    [mir-opt] mir-opt/issues/issue-75439.rs
    [mir-opt] mir-opt/issues/issue-75439.rs

test result: FAILED. 156 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 4.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:10:56
