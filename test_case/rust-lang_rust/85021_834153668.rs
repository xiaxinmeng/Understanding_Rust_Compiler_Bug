plain
.................................................................................................... 9500/11836
.................................................................................................... 9600/11836
......................................................i......i...................................... 9700/11836
.................................................................................................... 9800/11836
iiiiiii..iiiiii.i................................................................................... 9900/11836
.................................................................................................... 10100/11836
.................................................................................................... 10200/11836
.................................................................................................... 10300/11836
.................................................................................................... 10400/11836
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.189 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.44s

 finished in 2.516 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 25 tests
iiiiiiiiiiiii............

 finished in 3.265 seconds
Build completed successfully in 0:32:59
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
....F......................i................................
failures:

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
19       let mut _17: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
20       let mut _18: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
21       let mut _19: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _21: !;                          // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _22: core::panicking::AssertKind; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _23: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _24: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _25: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _26: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _27: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _21: ();                         // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _22: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _23: core::panicking::AssertKind; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _24: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _25: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _26: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _27: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _28: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
29       scope 1 {
30           debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
31           let _6: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14

33               debug _prev => _6;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
34               let _13: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
35               let _14: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-               let mut _28: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let mut _29: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
37               scope 4 {
38                   debug left_val => _13;   // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
39                   debug right_val => _14;  // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

81           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
82           _10 = &_1;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
83           StorageLive(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _28 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _29 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
85                                            // ty::Const
86                                            // + ty: &i32
87                                            // + val: Unevaluated(main, [], Some(promoted[0]))
88                                            // mir::Constant
88                                            // mir::Constant
89                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
90                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
-           _11 = _28;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _11 = _29;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
92           (_9.0: &i32) = move _10;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
93           (_9.1: &i32) = move _11;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
94           StorageDead(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

116           discriminant(_20) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
117           StorageLive(_21);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
118           StorageLive(_22);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _22 = const core::panicking::AssertKind::Eq; // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_23);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _23 = const core::panicking::AssertKind::Eq; // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
120                                            // ty::Const
121                                            // + ty: core::panicking::AssertKind
122                                            // + val: Value(Scalar(0x00))
123                                            // mir::Constant
123                                            // mir::Constant
124                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
125                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }
-           StorageLive(_23);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
127           StorageLive(_24);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _24 = _13;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _23 = _24;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
130           StorageLive(_25);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _25 = _13;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _24 = _25;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
131           StorageLive(_26);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _26 = _14;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _25 = _26;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
134           StorageLive(_27);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_27) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _27 = _14;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _26 = _27;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_28);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_28) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _24, move _26, move _28); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
137                                            // mir::Constant
138                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
139                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, std::option::Option<std::fmt::Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3554:25


failures:
    [mir-opt] mir-opt/issue-73223.rs
    [mir-opt] mir-opt/issue-73223.rs

test result: FAILED. 155 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.13s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:02:07
