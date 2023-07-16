plain
Warning: Skipping "src/test/mir-opt": not a regular file or directory
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 164 tests
..........................F........i.............................i..............i................... 100/164
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
....F...........F...........F.iF.F........F................F....

---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
17           StorageLive(_3);                 // scope 0 at $DIR/discriminant.rs:11:34: 11:44
18           ((_3 as Some).0: bool) = const true; // scope 0 at $DIR/discriminant.rs:11:34: 11:44
19           discriminant(_3) = 1;            // scope 0 at $DIR/discriminant.rs:11:34: 11:44
- -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- -         switchInt(move _4) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- +         _4 = const 1_isize;              // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- +         switchInt(const 1_isize) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+ -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:11:34: 11:44
+ -         switchInt(move _4) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:11:13: 11:44
+ +         _4 = const 1_isize;              // scope 0 at $DIR/discriminant.rs:11:34: 11:44
+ +         switchInt(const 1_isize) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:11:13: 11:44
25   
26       bb1: {


27           _2 = const 10_i32;               // scope 0 at $DIR/discriminant.rs:11:59: 11:61
-           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:13: 11:64
+           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:59: 11:61
30   
31       bb2: {


-           switchInt(((_3 as Some).0: bool)) -> [false: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:26: 11:30
+           switchInt(((_3 as Some).0: bool)) -> [false: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:13: 11:44
34   
35       bb3: {


36           _2 = const 42_i32;               // scope 0 at $DIR/discriminant.rs:11:47: 11:49
-           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:13: 11:64
+           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:47: 11:49
39   
40       bb4: {


thread '[mir-opt] mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
53           StorageLive(_2);                 // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
54           ((_2 as Some).0: i32) = const 1_i32; // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
55           discriminant(_2) = 1;            // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
-           _3 = const 1_isize;              // scope 0 at $DIR/issue-73223.rs:3:9: 3:16
-           goto -> bb2;                     // scope 0 at $DIR/issue-73223.rs:3:9: 3:16
+           _3 = const 1_isize;              // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
+           goto -> bb2;                     // scope 0 at $DIR/issue-73223.rs:2:17: 2:30
59   
60       bb1: {


thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/matches_reduce_branches.rs stdout ----
---- [mir-opt] mir-opt/matches_reduce_branches.rs stdout ----
5       debug bar => _1;                     // in scope 0 at $DIR/matches_reduce_branches.rs:7:8: 7:11
6       let mut _0: ();                      // return place in scope 0 at $DIR/matches_reduce_branches.rs:7:25: 7:25
7       let mut _2: isize;                   // in scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
- +     let mut _3: isize;                   // in scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
+ +     let mut _3: isize;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
10       bb0: {
10       bb0: {
-           _2 = discriminant(_1);           // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
- -         switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
+           _2 = discriminant(_1);           // scope 0 at $DIR/matches_reduce_branches.rs:8:17: 8:20
+ -         switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
13 -     }
15 -     bb1: {

21 -     }
22 - 
22 - 
23 -     bb3: {
- +         StorageLive(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
- +         _3 = move _2;                    // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
- +         StorageDead(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
+ +         StorageLive(_3);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ +         _3 = move _2;                    // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ +         StorageDead(_3);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
27           return;                          // scope 0 at $DIR/matches_reduce_branches.rs:11:2: 11:2
29   }


thread '[mir-opt] mir-opt/matches_reduce_branches.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_reduce_branches.foo.MatchBranchSimplification.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/simple-match.rs stdout ----
6 
7     bb0: {
7     bb0: {
8         FakeRead(ForMatchedPlace(None), _1); // scope 0 at $DIR/simple-match.rs:6:11: 6:12
-         switchInt(_1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simple-match.rs:7:9: 7:13
+         switchInt(_1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simple-match.rs:6:5: 6:12
11 
12     bb1: {

15 
15 
16     bb2: {
17         _0 = const 20_usize;             // scope 0 at $DIR/simple-match.rs:8:14: 8:16
-         goto -> bb4;                     // scope 0 at $DIR/simple-match.rs:6:5: 9:6
+         goto -> bb4;                     // scope 0 at $DIR/simple-match.rs:8:14: 8:16
20 
21     bb3: {


22         _0 = const 10_usize;             // scope 0 at $DIR/simple-match.rs:7:17: 7:19
-         goto -> bb4;                     // scope 0 at $DIR/simple-match.rs:6:5: 9:6
+         goto -> bb4;                     // scope 0 at $DIR/simple-match.rs:7:17: 7:19
25 
26     bb4: {


thread '[mir-opt] mir-opt/simple-match.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simple_match.match_bool.mir_map.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/simplify-arm-identity.rs stdout ----
---- [mir-opt] mir-opt/simplify-arm-identity.rs stdout ----
22           ((_1 as Foo).0: u8) = const 0_u8; // scope 0 at $DIR/simplify-arm-identity.rs:18:18: 18:29
23           discriminant(_1) = 0;            // scope 0 at $DIR/simplify-arm-identity.rs:18:18: 18:29
24           StorageLive(_2);                 // scope 1 at $DIR/simplify-arm-identity.rs:19:18: 22:6
-           _3 = const 0_isize;              // scope 1 at $DIR/simplify-arm-identity.rs:20:9: 20:20
-           goto -> bb3;                     // scope 1 at $DIR/simplify-arm-identity.rs:20:9: 20:20
+           _3 = const 0_isize;              // scope 1 at $DIR/simplify-arm-identity.rs:19:24: 19:25
+           goto -> bb3;                     // scope 1 at $DIR/simplify-arm-identity.rs:19:18: 19:25
28   
29       bb1: {


30           ((_2 as Foo).0: u8) = const 0_u8; // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
31           discriminant(_2) = 0;            // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
-           goto -> bb4;                     // scope 1 at $DIR/simplify-arm-identity.rs:19:18: 22:6
+           goto -> bb4;                     // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
34   
35       bb2: {


45           discriminant(_2) = 0;            // scope 3 at $DIR/simplify-arm-identity.rs:20:24: 20:35
46           StorageDead(_5);                 // scope 3 at $DIR/simplify-arm-identity.rs:20:34: 20:35
47           StorageDead(_4);                 // scope 1 at $DIR/simplify-arm-identity.rs:20:34: 20:35
-           goto -> bb4;                     // scope 1 at $DIR/simplify-arm-identity.rs:19:18: 22:6
+           goto -> bb4;                     // scope 1 at $DIR/simplify-arm-identity.rs:20:34: 20:35
50   
51       bb4: {


thread '[mir-opt] mir-opt/simplify-arm-identity.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_arm_identity.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/matches_u8.rs stdout ----
---- [mir-opt] mir-opt/matches_u8.rs stdout ----
7       let mut _2: isize;                   // in scope 0 at $DIR/matches_u8.rs:13:9: 13:13
9       bb0: {
9       bb0: {
-           _2 = discriminant(_1);           // scope 0 at $DIR/matches_u8.rs:13:9: 13:13
-           switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/matches_u8.rs:13:9: 13:13
+           _2 = discriminant(_1);           // scope 0 at $DIR/matches_u8.rs:12:11: 12:12
+           switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/matches_u8.rs:12:5: 12:12
13   
14       bb1: {


15           _0 = const 1_u8;                 // scope 0 at $DIR/matches_u8.rs:14:17: 14:18
-           goto -> bb3;                     // scope 0 at $DIR/matches_u8.rs:12:5: 15:6
+           goto -> bb3;                     // scope 0 at $DIR/matches_u8.rs:14:17: 14:18
18   
19       bb2: {


20           _0 = const 0_u8;                 // scope 0 at $DIR/matches_u8.rs:13:17: 13:18
-           goto -> bb3;                     // scope 0 at $DIR/matches_u8.rs:12:5: 15:6
+           goto -> bb3;                     // scope 0 at $DIR/matches_u8.rs:13:17: 13:18
23   
24       bb3: {


thread '[mir-opt] mir-opt/matches_u8.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_u8.exhaustive_match.MatchBranchSimplification.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
15       }
16   
17       bb0: {
17       bb0: {
- -         _5 = const false;                // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:9: 5:13
- -         _5 = const true;                 // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:9: 5:13
- -         _2 = discriminant(_1);           // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:9: 5:13
+ -         _5 = const false;                // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:11: 4:12
+ -         _5 = const true;                 // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:11: 4:12
+ -         _2 = discriminant(_1);           // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:11: 4:12
21           _0 = move _1;                    // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
22 -         _6 = discriminant(_1);           // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:1: 8:2
23           return;                          // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:2: 8:2

thread '[mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_removes_unused_discriminant_reads.map.SimplifyLocals.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/while_let_loops.rs stdout ----
---- [mir-opt] mir-opt/while_let_loops.rs stdout ----
18           _1 = const 0_i32;                // scope 0 at $DIR/while_let_loops.rs:6:18: 6:19
19           StorageLive(_3);                 // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
20           discriminant(_3) = 0;            // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
- -         _4 = discriminant(_3);           // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
- -         switchInt(move _4) -> [1_isize: bb2, otherwise: bb1]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
- +         _4 = const 0_isize;              // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
- +         switchInt(const 0_isize) -> [1_isize: bb2, otherwise: bb1]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
+ -         _4 = discriminant(_3);           // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
+ -         switchInt(move _4) -> [1_isize: bb2, otherwise: bb1]; // scope 1 at $DIR/while_let_loops.rs:7:5: 7:32
+ +         _4 = const 0_isize;              // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
+ +         switchInt(const 0_isize) -> [1_isize: bb2, otherwise: bb1]; // scope 1 at $DIR/while_let_loops.rs:7:5: 7:32
26   
27       bb1: {

30       }
30       }
31   
32       bb2: {
-           switchInt(((_3 as Some).0: u32)) -> [0_u32: bb3, otherwise: bb1]; // scope 1 at $DIR/while_let_loops.rs:7:20: 7:24
+           switchInt(((_3 as Some).0: u32)) -> [0_u32: bb3, otherwise: bb1]; // scope 1 at $DIR/while_let_loops.rs:7:5: 7:32
35   
36       bb3: {


thread '[mir-opt] mir-opt/while_let_loops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/while_let_loops.change_loop_body.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25

failures:
    [mir-opt] mir-opt/const_prop/discriminant.rs
    [mir-opt] mir-opt/issue-73223.rs
---
test result: FAILED. 152 passed; 8 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.77s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:55
