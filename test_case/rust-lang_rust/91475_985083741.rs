plain
....F...........................i.................................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/early_otherwise_branch_noopt.rs stdout ----
thread '[mir-opt] mir-opt/early_otherwise_branch_noopt.rs' panicked at 'the mir dump file for early_otherwise_branch_noopt.noopt1.EarlyOtherwiseBranch.before.mir does not exist (requested in /checkout/src/test/mir-opt/early_otherwise_branch_noopt.rs)', src/tools/compiletest/src/runtest.rs:3373:17

---- [mir-opt] mir-opt/issues/issue-59352.rs stdout ----
---- [mir-opt] mir-opt/issues/issue-59352.rs stdout ----
3 fn num_to_digit(_1: char) -> u32 {
4     debug num => _1;                     // in scope 0 at $DIR/issue-59352.rs:12:21: 12:24
5     let mut _0: u32;                     // return place in scope 0 at $DIR/issue-59352.rs:12:35: 12:38
-     let mut _2: char;                    // in scope 0 at $DIR/issue-59352.rs:14:8: 14:11
-     let mut _3: std::option::Option<u32>; // in scope 0 at $DIR/issue-59352.rs:14:26: 14:41
-     let mut _4: char;                    // in scope 0 at $DIR/issue-59352.rs:14:26: 14:29
-     let mut _5: u32;                     // in scope 0 at $DIR/issue-59352.rs:14:8: 14:23
-     let mut _11: isize;                  // in scope 0 at $DIR/issue-59352.rs:14:8: 14:23
+     let mut _2: bool;                    // in scope 0 at $DIR/issue-59352.rs:14:8: 14:23
+     let mut _3: char;                    // in scope 0 at $DIR/issue-59352.rs:14:8: 14:11
+     let mut _4: std::option::Option<u32>; // in scope 0 at $DIR/issue-59352.rs:14:26: 14:41
+     let mut _5: char;                    // in scope 0 at $DIR/issue-59352.rs:14:26: 14:29
+     let mut _6: u32;                     // in scope 0 at $DIR/issue-59352.rs:14:8: 14:23
11     scope 1 (inlined char::methods::<impl char>::is_digit) { // at $DIR/issue-59352.rs:14:8: 14:23
-         debug self => _2;                // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         debug radix => _5;               // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         let mut _6: &std::option::Option<u32>; // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         let _7: std::option::Option<u32>; // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         let mut _8: char;                // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         debug self => _3;                // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         debug radix => _6;               // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         let mut _7: &std::option::Option<u32>; // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         let _8: std::option::Option<u32>; // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         let mut _9: char;                // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
17         scope 2 (inlined Option::<u32>::is_some) { // at $DIR/issue-59352.rs:14:8: 14:23
-             debug self => _6;            // in scope 2 at $DIR/issue-59352.rs:14:8: 14:23
-             let mut _9: isize;           // in scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+             debug self => _7;            // in scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+             let mut _10: isize;          // in scope 2 at $DIR/issue-59352.rs:14:8: 14:23
21     }
21     }
22     scope 3 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue-59352.rs:14:26: 14:50

-         debug self => _3;                // in scope 3 at $DIR/issue-59352.rs:14:26: 14:50
-         let mut _10: isize;              // in scope 3 at $DIR/issue-59352.rs:14:26: 14:50
+         debug self => _4;                // in scope 3 at $DIR/issue-59352.rs:14:26: 14:50
+         let mut _11: isize;              // in scope 3 at $DIR/issue-59352.rs:14:26: 14:50
25         scope 4 {
26             debug val => _0;             // in scope 4 at $DIR/issue-59352.rs:14:26: 14:50

28     }
29 
30     bb0: {
30     bb0: {
-         StorageLive(_2);                 // scope 0 at $DIR/issue-59352.rs:14:8: 14:11
-         _2 = _1;                         // scope 0 at $DIR/issue-59352.rs:14:8: 14:11
-         StorageLive(_5);                 // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
-         _5 = const 8_u32;                // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
-         StorageLive(_6);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         StorageLive(_2);                 // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
+         StorageLive(_3);                 // scope 0 at $DIR/issue-59352.rs:14:8: 14:11
+         _3 = _1;                         // scope 0 at $DIR/issue-59352.rs:14:8: 14:11
+         StorageLive(_6);                 // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
+         _6 = const 8_u32;                // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
36         StorageLive(_7);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
37         StorageLive(_8);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         _8 = _2;                         // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         _7 = char::methods::<impl char>::to_digit(move _8, const 8_u32) -> bb5; // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         StorageLive(_9);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         _9 = _3;                         // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         _8 = char::methods::<impl char>::to_digit(move _9, const 8_u32) -> bb5; // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
40                                          // mir::Constant
41                                          // + span: $DIR/issue-59352.rs:14:8: 14:23
42                                          // + literal: Const { ty: fn(char, u32) -> std::option::Option<u32> {std::char::methods::<impl char>::to_digit}, val: Value(Scalar(<ZST>)) }
43     }
44 
45     bb1: {
45     bb1: {
-         StorageDead(_11);                // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
-         StorageLive(_3);                 // scope 0 at $DIR/issue-59352.rs:14:26: 14:41
-         StorageLive(_4);                 // scope 0 at $DIR/issue-59352.rs:14:26: 14:29
-         _4 = _1;                         // scope 0 at $DIR/issue-59352.rs:14:26: 14:29
-         _3 = char::methods::<impl char>::to_digit(move _4, const 8_u32) -> bb2; // scope 0 at $DIR/issue-59352.rs:14:26: 14:41
+         StorageLive(_4);                 // scope 0 at $DIR/issue-59352.rs:14:26: 14:41
+         StorageLive(_5);                 // scope 0 at $DIR/issue-59352.rs:14:26: 14:29
+         _5 = _1;                         // scope 0 at $DIR/issue-59352.rs:14:26: 14:29
+         _4 = char::methods::<impl char>::to_digit(move _5, const 8_u32) -> bb2; // scope 0 at $DIR/issue-59352.rs:14:26: 14:41
51                                          // mir::Constant
52                                          // + span: $DIR/issue-59352.rs:14:30: 14:38
53                                          // + literal: Const { ty: fn(char, u32) -> std::option::Option<u32> {std::char::methods::<impl char>::to_digit}, val: Value(Scalar(<ZST>)) }
54     }
55 
56     bb2: {
56     bb2: {
-         StorageDead(_4);                 // scope 0 at $DIR/issue-59352.rs:14:40: 14:41
-         StorageLive(_10);                // scope 0 at $DIR/issue-59352.rs:14:26: 14:50
-         _10 = discriminant(_3);          // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
-         switchInt(move _10) -> [0_isize: bb6, 1_isize: bb8, otherwise: bb7]; // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
+         StorageDead(_5);                 // scope 0 at $DIR/issue-59352.rs:14:40: 14:41
+         StorageLive(_11);                // scope 0 at $DIR/issue-59352.rs:14:26: 14:50
+         _11 = discriminant(_4);          // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
+         switchInt(move _11) -> [0_isize: bb9, 1_isize: bb11, otherwise: bb10]; // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
62 
63     bb3: {


-         StorageDead(_11);                // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
65         _0 = const 0_u32;                // scope 0 at $DIR/issue-59352.rs:14:60: 14:61
66         goto -> bb4;                     // scope 0 at $DIR/issue-59352.rs:14:5: 14:63

68 
69     bb4: {
69     bb4: {
+         StorageDead(_2);                 // scope 0 at $DIR/issue-59352.rs:14:62: 14:63
70         return;                          // scope 0 at $DIR/issue-59352.rs:15:2: 15:2
72 

73     bb5: {
73     bb5: {
-         _6 = &_7;                        // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         StorageDead(_8);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         StorageLive(_9);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         _9 = discriminant((*_6));        // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
-         StorageLive(_11);                // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
-         _11 = move _9;                   // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+         _7 = &_8;                        // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
80         StorageDead(_9);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         StorageDead(_6);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         StorageDead(_7);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         StorageDead(_5);                 // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
-         StorageDead(_2);                 // scope 0 at $DIR/issue-59352.rs:14:22: 14:23
-         switchInt(move _11) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
+         StorageLive(_10);                // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         _10 = discriminant((*_7));       // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+         switchInt(move _10) -> [1_isize: bb8, otherwise: bb7]; // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
87 
88     bb6: {


+         StorageDead(_10);                // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         StorageDead(_7);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         StorageDead(_8);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
+         StorageDead(_6);                 // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
+         StorageDead(_3);                 // scope 0 at $DIR/issue-59352.rs:14:22: 14:23
+         switchInt(move _2) -> [false: bb3, otherwise: bb1]; // scope 0 at $DIR/issue-59352.rs:14:8: 14:23
+ 
+     bb7: {
+     bb7: {
+         _2 = const false;                // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+         goto -> bb6;                     // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+ 
+     bb8: {
+     bb8: {
+         _2 = const true;                 // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+         goto -> bb6;                     // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+ 
+     bb9: {
+     bb9: {
89         core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
90                                          // mir::Constant
91                                          // + span: $DIR/issue-59352.rs:14:26: 14:50

98                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [99, 97, 108, 108, 101, 100, 32, 96, 79, 112, 116, 105, 111, 110, 58, 58, 117, 110, 119, 114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 32, 96, 78, 111, 110, 101, 96, 32, 118, 97, 108, 117, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8796093022207], len: Size { raw: 43 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 43 }) }
100 
-     bb7: {
+     bb10: {
+     bb10: {
102         unreachable;                     // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
104 

-     bb8: {
-     bb8: {
-         _0 = move ((_3 as Some).0: u32); // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
-         StorageDead(_10);                // scope 0 at $DIR/issue-59352.rs:14:26: 14:50
-         StorageDead(_3);                 // scope 0 at $DIR/issue-59352.rs:14:49: 14:50
+     bb11: {
+         _0 = move ((_4 as Some).0: u32); // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
+         StorageDead(_11);                // scope 0 at $DIR/issue-59352.rs:14:26: 14:50
+         StorageDead(_4);                 // scope 0 at $DIR/issue-59352.rs:14:49: 14:50
109         goto -> bb4;                     // scope 0 at $DIR/issue-59352.rs:14:5: 14:63
111 }


thread '[mir-opt] mir-opt/issues/issue-59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3359:25

failures:
    [mir-opt] mir-opt/early_otherwise_branch_noopt.rs
    [mir-opt] mir-opt/issues/issue-59352.rs
    [mir-opt] mir-opt/issues/issue-59352.rs

test result: FAILED. 161 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out; finished in 4.70s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:33
