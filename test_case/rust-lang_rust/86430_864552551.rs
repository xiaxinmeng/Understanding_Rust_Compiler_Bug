plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
........F....................i................................
failures:

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
6       let _1: i32;                         // in scope 0 at $DIR/issue-73223.rs:2:9: 2:14
7       let mut _2: std::option::Option<i32>; // in scope 0 at $DIR/issue-73223.rs:2:23: 2:30
8       let _3: i32;                         // in scope 0 at $DIR/issue-73223.rs:3:14: 3:15
-       let mut _5: i32;                     // in scope 0 at $DIR/issue-73223.rs:7:22: 7:27
-       let mut _6: (&i32, &i32);            // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _7: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _8: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _11: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _12: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _13: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _5: (&i32, &i32);            // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _6: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _9: bool;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _10: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _11: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _13: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _14: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
16       let mut _15: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
17       let _16: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _17: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _18: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _19: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _17: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
21       scope 1 {
22           debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
23           let _4: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14
24           scope 3 {
24           scope 3 {
25               debug _prev => _4;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
-               let _9: &i32;                // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-               let _10: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-               let mut _20: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let _7: &i32;                // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let _8: &i32;                // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let mut _18: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
29               scope 4 {
-                   debug left_val => _9;    // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                   debug right_val => _10;  // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                   let _14: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug left_val => _7;    // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug right_val => _8;   // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   let _12: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
33                   scope 5 {
-                       debug kind => _14;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                       debug kind => _12;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
36               }
37           }


47           discriminant(_2) = 1;            // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
48           StorageLive(_3);                 // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
49           _3 = ((_2 as Some).0: i32);      // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
-           _1 = _3;                         // scope 2 at $DIR/issue-73223.rs:3:20: 3:21
+           _1 = ((_2 as Some).0: i32);      // scope 2 at $DIR/issue-73223.rs:3:20: 3:21
51           StorageDead(_3);                 // scope 0 at $DIR/issue-73223.rs:3:20: 3:21
52           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:5:6: 5:7
53           StorageLive(_4);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14

-           StorageLive(_5);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
-           _5 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
-           ((_4 as Some).0: i32) = move _5; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
+           ((_4 as Some).0: i32) = _1;      // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
57           discriminant(_4) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
-           StorageDead(_5);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
+           StorageLive(_5);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
59           StorageLive(_6);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _7 = &_1;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _20 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _6 = &_1;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _18 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
64                                            // ty::Const
65                                            // + ty: &i32
66                                            // + val: Unevaluated(main, [], Some(promoted[0]))
67                                            // mir::Constant
67                                            // mir::Constant
68                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
69                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
-           _8 = _20;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_6.0: &i32) = move _7;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_6.1: &i32) = move _8;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _9 = (_6.0: &i32);               // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _10 = (_6.1: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_5.0: &i32) = move _6;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_5.1: &i32) = _18;              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_6);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _7 = (_5.0: &i32);               // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _8 = (_5.1: &i32);               // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_9);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_10);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
79           StorageLive(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_12);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_13);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _13 = (*_9);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _12 = Eq(move _13, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_13);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _11 = Not(move _12);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_12);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           switchInt(move _11) -> [false: bb2, otherwise: bb1]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _11 = (*_7);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _10 = Eq(move _11, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _9 = Not(move _10);              // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_10);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           switchInt(move _9) -> [false: bb2, otherwise: bb1]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
89   
90       bb1: {


-           StorageLive(_14);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_14) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_12);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_12) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_13);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_14);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _14 = _7;                        // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _13 = _14;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
93           StorageLive(_15);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
94           StorageLive(_16);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _16 = _9;                        // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _16 = _8;                        // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
96           _15 = _16;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
97           StorageLive(_17);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_18);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _18 = _10;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _17 = _18;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_19);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_19) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _15, move _17, move _19); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_17) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _13, move _15, move _17); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
104                                            // mir::Constant
105                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
106                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, std::option::Option<std::fmt::Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }
113       }
114   
115       bb2: {
115       bb2: {
-           StorageDead(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_6);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_9);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_5);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
120           StorageDead(_4);                 // scope 1 at $DIR/issue-73223.rs:9:1: 9:2
121           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
122           return;                          // scope 0 at $DIR/issue-73223.rs:9:2: 9:2

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.PreCodegen.32bit.diff', src/tools/compiletest/src/runtest.rs:3573:25


failures:
    [mir-opt] mir-opt/issue-73223.rs
    [mir-opt] mir-opt/issue-73223.rs

test result: FAILED. 157 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 3.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:43
