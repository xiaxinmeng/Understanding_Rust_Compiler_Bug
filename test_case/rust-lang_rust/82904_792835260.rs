plain
.................................................................................................... 9300/11531
.................................................................................................... 9400/11531
.......................................................................i......i..................... 9500/11531
.................................................................................................... 9600/11531
..........iiiiiii..iiiiii..i........................................................................ 9700/11531
.................................................................................................... 9900/11531
.................................................................................................... 10000/11531
.................................................................................................... 10100/11531
.................................................................................................... 10200/11531
---
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 158 tests
..................................i..................................F.........i.F.................. 100/158
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.........................i..........F.......F.............

---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
20     let _3: std::string::String;         // in scope 0 at $DIR/generator-drop-cleanup.rs:11:13: 11:15
21     let _4: ();                          // in scope 0 at $DIR/generator-drop-cleanup.rs:12:9: 12:14
22     let mut _5: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:12:9: 12:14
-     let mut _7: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:18: 10:18
-     let mut _8: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
-     let mut _9: u32;                     // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+     let mut _6: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:18: 10:18
+     let mut _7: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+     let mut _8: u32;                     // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
26     scope 1 {
27         debug _s => (((*_1) as variant#3).0: std::string::String); // in scope 1 at $DIR/generator-drop-cleanup.rs:11:13: 11:15


29     scope 2 (inlined String::new) {      // at $DIR/generator-drop-cleanup.rs:11:18: 11:31
-         let mut _6: std::vec::Vec<u8>;   // in scope 2 at $DIR/generator-drop-cleanup.rs:11:18: 11:31
31         scope 3 (inlined Vec::<u8>::new) { // at $DIR/generator-drop-cleanup.rs:11:18: 11:31
33     }

34 
35     bb0: {
35     bb0: {
-         _9 = discriminant((*_1));        // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
-         switchInt(move _9) -> [0_u32: bb7, 3_u32: bb10, otherwise: bb11]; // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         _8 = discriminant((*_1));        // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         switchInt(move _8) -> [0_u32: bb7, 3_u32: bb10, otherwise: bb11]; // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
39 
40     bb1: {


thread '[mir-opt] mir-opt/generator-drop-cleanup.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_drop_cleanup.main-{closure#0}.generator_drop.0.mir', src/tools/compiletest/src/runtest.rs:3514:25

---- [mir-opt] mir-opt/inline/inline-generator.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-generator.rs stdout ----
7       let mut _2: std::pin::Pin<&mut impl std::ops::Generator<bool>>; // in scope 0 at $DIR/inline-generator.rs:9:14: 9:32
8       let mut _3: &mut impl std::ops::Generator<bool>; // in scope 0 at $DIR/inline-generator.rs:9:23: 9:31
9       let mut _4: impl std::ops::Generator<bool>; // in scope 0 at $DIR/inline-generator.rs:9:28: 9:31
- +     let mut _7: bool;                    // in scope 0 at $DIR/inline-generator.rs:9:14: 9:46
+ +     let mut _6: bool;                    // in scope 0 at $DIR/inline-generator.rs:9:14: 9:46
11       scope 1 {
12           debug _r => _1;                  // in scope 1 at $DIR/inline-generator.rs:9:9: 9:11

19 +         scope 4 {
19 +         scope 4 {
20 +             scope 5 (inlined Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:41 {bool, i32}]>::new_unchecked) { // at $DIR/inline-generator.rs:9:14: 9:32
21 +                 debug pointer => _5;     // in scope 5 at $DIR/inline-generator.rs:9:14: 9:32
- +                 let mut _6: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41 {bool, i32}]; // in scope 5 at $DIR/inline-generator.rs:9:14: 9:32
23 +             }
24 +         }
25 +     }

26 +     scope 6 (inlined g::{closure#0}) {   // at $DIR/inline-generator.rs:9:14: 9:46
- +         debug a => _8;                   // in scope 6 at $DIR/inline-generator.rs:9:14: 9:46
- +         let mut _8: bool;                // in scope 6 at $DIR/inline-generator.rs:9:14: 9:46
- +         let mut _9: u32;                 // in scope 6 at $DIR/inline-generator.rs:9:14: 9:46
+ +         debug a => _7;                   // in scope 6 at $DIR/inline-generator.rs:9:14: 9:46
+ +         let mut _7: bool;                // in scope 6 at $DIR/inline-generator.rs:9:14: 9:46
+ +         let mut _8: u32;                 // in scope 6 at $DIR/inline-generator.rs:9:14: 9:46
30 +     }
32       bb0: {

53 -     bb2: {
53 -     bb2: {
54 +         StorageLive(_5);                 // scope 4 at $DIR/inline-generator.rs:9:14: 9:32
55 +         _5 = move _3;                    // scope 4 at $DIR/inline-generator.rs:9:14: 9:32
- +         StorageLive(_6);                 // scope 5 at $DIR/inline-generator.rs:9:14: 9:32
- +         _6 = move _5;                    // scope 5 at $DIR/inline-generator.rs:9:14: 9:32
- +         (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41 {bool, i32}]) = move _6; // scope 5 at $DIR/inline-generator.rs:9:14: 9:32
- +         StorageDead(_6);                 // scope 5 at $DIR/inline-generator.rs:9:14: 9:32
+ +         (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41 {bool, i32}]) = move _5; // scope 5 at $DIR/inline-generator.rs:9:14: 9:32
60 +         StorageDead(_5);                 // scope 4 at $DIR/inline-generator.rs:9:14: 9:32
61           StorageDead(_3);                 // scope 0 at $DIR/inline-generator.rs:9:31: 9:32
62 -         _1 = <impl Generator<bool> as Generator<bool>>::resume(move _2, const false) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
63 -                                          // mir::Constant
63 -                                          // mir::Constant
64 -                                          // + span: $DIR/inline-generator.rs:9:33: 9:39
65 -                                          // + literal: Const { ty: for<'r> fn(std::pin::Pin<&'r mut impl std::ops::Generator<bool>>, bool) -> std::ops::GeneratorState<<impl std::ops::Generator<bool> as std::ops::Generator<bool>>::Yield, <impl std::ops::Generator<bool> as std::ops::Generator<bool>>::Return> {<impl std::ops::Generator<bool> as std::ops::Generator<bool>>::resume}, val: Value(Scalar(<ZST>)) }
+ +         StorageLive(_6);                 // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
+ +         _6 = const false;                // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
66 +         StorageLive(_7);                 // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
- +         _7 = const false;                // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
68 +         StorageLive(_8);                 // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
- +         StorageLive(_9);                 // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
- +         _9 = discriminant((*(_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41 {bool, i32}]))); // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
- +         switchInt(move _9) -> [0_u32: bb3, 1_u32: bb8, 3_u32: bb7, otherwise: bb9]; // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
+ +         _8 = discriminant((*(_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41 {bool, i32}]))); // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
+ +         switchInt(move _8) -> [0_u32: bb3, 1_u32: bb8, 3_u32: bb7, otherwise: bb9]; // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
73   
74 -     bb3: {

75 +     bb1: {
75 +     bb1: {
- +         StorageDead(_9);                 // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
77 +         StorageDead(_8);                 // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
78 +         StorageDead(_7);                 // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
+ +         StorageDead(_6);                 // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
79           StorageDead(_2);                 // scope 0 at $DIR/inline-generator.rs:9:45: 9:46
80           StorageDead(_4);                 // scope 0 at $DIR/inline-generator.rs:9:46: 9:47
81           _0 = const ();                   // scope 0 at $DIR/inline-generator.rs:8:11: 10:2
89 +     }
90 + 
91 +     bb3: {
91 +     bb3: {
- +         _8 = move _7;                    // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
- +         switchInt(move _8) -> [false: bb5, otherwise: bb4]; // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
+ +         _7 = move _6;                    // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
+ +         switchInt(move _7) -> [false: bb5, otherwise: bb4]; // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
94 +     }
96 +     bb4: {

110 +     }
111 + 
111 + 
112 +     bb7: {
- +         ((_1 as Complete).0: bool) = move _7; // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
+ +         ((_1 as Complete).0: bool) = move _6; // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
114 +         discriminant(_1) = 1;            // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
115 +         discriminant((*(_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41 {bool, i32}]))) = 1; // scope 6 at $DIR/inline-generator.rs:9:14: 9:46
116 +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:41: 15:41

thread '[mir-opt] mir-opt/inline/inline-generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3514:25
---- [mir-opt] mir-opt/simplify-arm.rs stdout ----
---- [mir-opt] mir-opt/simplify-arm.rs stdout ----
29               scope 8 (inlined <Result<u8, i32> as Try>::from_error) { // at $DIR/simplify-arm.rs:24:13: 24:15
30 -                 debug v => _8;           // in scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
31 +                 debug v => ((_0 as Err).0: i32); // in scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
-                   let mut _12: i32;        // in scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
34           }
35       }


82 -         _9 = _6;                         // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
83 -         _8 = move _9;                    // scope 7 at $DIR/simplify-arm.rs:24:14: 24:15
84 -         StorageDead(_9);                 // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
- -         StorageLive(_12);                // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
- -         _12 = move _8;                   // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
- -         ((_0 as Err).0: i32) = move _12; // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+ -         ((_0 as Err).0: i32) = move _8;  // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
88 -         discriminant(_0) = 1;            // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
- -         StorageDead(_12);                // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
90 -         StorageDead(_8);                 // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
91 -         StorageDead(_6);                 // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
92 +         _0 = move _3;                    // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15

thread '[mir-opt] mir-opt/simplify-arm.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_arm.id_try.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3514:25
---- [mir-opt] mir-opt/simplify_try.rs stdout ----
---- [mir-opt] mir-opt/simplify_try.rs stdout ----
29               scope 8 (inlined <Result<u32, i32> as Try>::from_error) { // at $DIR/simplify_try.rs:8:13: 8:15
30 -                 debug v => _8;           // in scope 8 at $DIR/simplify_try.rs:8:13: 8:15
31 +                 debug v => ((_0 as Err).0: i32); // in scope 8 at $DIR/simplify_try.rs:8:13: 8:15
-                   let mut _12: i32;        // in scope 8 at $DIR/simplify_try.rs:8:13: 8:15
34           }
35       }


78 -         _9 = _6;                         // scope 3 at $DIR/simplify_try.rs:8:14: 8:15
79 -         _8 = move _9;                    // scope 7 at $DIR/simplify_try.rs:8:14: 8:15
80 -         StorageDead(_9);                 // scope 3 at $DIR/simplify_try.rs:8:14: 8:15
- -         StorageLive(_12);                // scope 8 at $DIR/simplify_try.rs:8:13: 8:15
- -         _12 = move _8;                   // scope 8 at $DIR/simplify_try.rs:8:13: 8:15
- -         ((_0 as Err).0: i32) = move _12; // scope 8 at $DIR/simplify_try.rs:8:13: 8:15
+ -         ((_0 as Err).0: i32) = move _8;  // scope 8 at $DIR/simplify_try.rs:8:13: 8:15
84 -         discriminant(_0) = 1;            // scope 8 at $DIR/simplify_try.rs:8:13: 8:15
- -         StorageDead(_12);                // scope 8 at $DIR/simplify_try.rs:8:13: 8:15
86 -         StorageDead(_8);                 // scope 3 at $DIR/simplify_try.rs:8:14: 8:15
87 -         StorageDead(_6);                 // scope 0 at $DIR/simplify_try.rs:8:14: 8:15
88 +         _0 = move _3;                    // scope 8 at $DIR/simplify_try.rs:8:13: 8:15

thread '[mir-opt] mir-opt/simplify_try.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_try.try_identity.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3514:25

failures:
    [mir-opt] mir-opt/generator-drop-cleanup.rs
    [mir-opt] mir-opt/inline/inline-generator.rs
    [mir-opt] mir-opt/inline/inline-generator.rs
    [mir-opt] mir-opt/simplify-arm.rs
    [mir-opt] mir-opt/simplify_try.rs

test result: FAILED. 151 passed; 4 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:38
