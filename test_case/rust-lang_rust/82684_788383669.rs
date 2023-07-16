plain
.................................................................................................... 9300/11515
.................................................................................................... 9400/11515
..................................................................i......i.......................... 9500/11515
.................................................................................................... 9600/11515
.....iiiiiii..iiiiii.i.............................................................................. 9700/11515
.................................................................................................... 9900/11515
.................................................................................................... 10000/11515
.................................................................................................... 10100/11515
.................................................................................................... 10200/11515
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.077 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.444 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiii.........

 finished in 0.639 seconds
Build completed successfully in 0:30:43
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
....F....................i................................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
6       let _1: i32;                         // in scope 0 at $DIR/issue-73223.rs:2:9: 2:14
7       let mut _2: std::option::Option<i32>; // in scope 0 at $DIR/issue-73223.rs:2:23: 2:30
8       let _3: i32;                         // in scope 0 at $DIR/issue-73223.rs:3:14: 3:15
-       let mut _5: (&i32, &i32);            // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _6: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _7: bool;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _8: bool;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _9: i32;                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _11: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _12: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _13: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _5: i32;                     // in scope 0 at $DIR/issue-73223.rs:7:22: 7:27
+       let mut _6: (&i32, &i32);            // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _7: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _8: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _11: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _12: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _13: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _15: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _16: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _17: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _18: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _19: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
17       scope 1 {
18           debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
19           let _4: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14
20           scope 3 {
20           scope 3 {
21               debug _prev => _4;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
+               let _9: &i32;                // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let _10: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let mut _20: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
22               scope 4 {
-                   debug left_val => _11;   // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                   debug right_val => _12;  // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                   let _10: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug left_val => _9;    // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug right_val => _10;  // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   let _14: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
26                   scope 5 {
-                       debug kind => _10;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                       debug kind => _14;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
29               }
30           }


43           _1 = _3;                         // scope 2 at $DIR/issue-73223.rs:3:20: 3:21
44           StorageDead(_3);                 // scope 0 at $DIR/issue-73223.rs:3:20: 3:21
45           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:5:6: 5:7
-           ((_4 as Some).0: i32) = _1;      // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+           StorageLive(_4);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
+           StorageLive(_5);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+           _5 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+           ((_4 as Some).0: i32) = move _5; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
47           discriminant(_4) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
-           (_5.0: &i32) = &_1;              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _6 = const main::promoted[0];    // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_5);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
+           StorageLive(_6);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _7 = &_1;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _20 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
50                                            // ty::Const
51                                            // + ty: &i32
52                                            // + val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, [], Some(promoted[0]))
53                                            // mir::Constant
53                                            // mir::Constant
54                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
55                                            // + literal: Const { ty: &i32, val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, [], Some(promoted[0])) }
-           (_5.1: &i32) = move _6;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _11 = (_5.0: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _12 = (_5.1: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_7);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_8);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_9);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _9 = (*_11);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _8 = Eq(move _9, const 1_i32);   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_9);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _7 = Not(move _8);               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_8);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           switchInt(move _7) -> [false: bb2, otherwise: bb1]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _8 = _20;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_6.0: &i32) = move _7;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_6.1: &i32) = move _8;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _9 = (_6.0: &i32);               // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _10 = (_6.1: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_12);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_13);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _13 = (*_9);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _12 = Eq(move _13, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_13);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _11 = Not(move _12);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_12);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           switchInt(move _11) -> [false: bb2, otherwise: bb1]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
69   
70       bb1: {


-           StorageLive(_10);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_10) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_13);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_13) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _11, move _12, move _13); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_14);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_14) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_15);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_16);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _16 = _9;                        // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _15 = _16;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_17);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_18);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _18 = _10;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _17 = _18;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_19);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_19) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _15, move _17, move _19); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
76                                            // mir::Constant
77                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
78                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, std::option::Option<std::fmt::Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }
85       }
86   
87       bb2: {
87       bb2: {
-           StorageDead(_7);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_6);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
89           _0 = const ();                   // scope 0 at $DIR/issue-73223.rs:1:11: 9:2
+           StorageDead(_4);                 // scope 1 at $DIR/issue-73223.rs:9:1: 9:2
90           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
91           return;                          // scope 0 at $DIR/issue-73223.rs:9:2: 9:2


thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.PreCodegen.32bit.diff', src/tools/compiletest/src/runtest.rs:3458:25


failures:
    [mir-opt] mir-opt/issue-73223.rs
    [mir-opt] mir-opt/issue-73223.rs

test result: FAILED. 153 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.17s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:50
