plain
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 165 tests
...................................i....................................F.......i................... 100/165
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....F...........F..............i..F............F....F............

---- [mir-opt] mir-opt/generator-tiny.rs stdout ----
---- [mir-opt] mir-opt/generator-tiny.rs stdout ----
10     storage_conflicts: BitMatrix(0x0) {},
11 } */
12 
- fn main::{closure#0}(_1: Pin<&mut [generator@$DIR/generator-tiny.rs:19:16: 25:6]>, _2: u8) -> GeneratorState<(), ()> {
+ fn main::{closure#0}(_1: Pin<&mut [generator@$DIR/generator-tiny.rs:19:16: 25:6]>, _2: u8) -> GeneratorState<(), !> {
14     debug _x => _10;                     // in scope 0 at $DIR/generator-tiny.rs:19:17: 19:19
-     let mut _0: std::ops::GeneratorState<(), ()>; // return place in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
+     let mut _0: std::ops::GeneratorState<(), !>; // return place in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
16     let _3: HasDrop;                     // in scope 0 at $DIR/generator-tiny.rs:20:13: 20:15
17     let mut _4: !;                       // in scope 0 at $DIR/generator-tiny.rs:21:9: 24:10
18     let mut _5: ();                      // in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6

19     let _6: u8;                          // in scope 0 at $DIR/generator-tiny.rs:22:13: 22:18
20     let mut _7: ();                      // in scope 0 at $DIR/generator-tiny.rs:22:13: 22:18
21     let _8: ();                          // in scope 0 at $DIR/generator-tiny.rs:23:13: 23:21
-     let mut _9: ();                      // in scope 0 at $DIR/generator-tiny.rs:19:25: 19:25
+     let mut _9: !;                       // in scope 0 at $DIR/generator-tiny.rs:19:25: 19:25
23     let _10: u8;                         // in scope 0 at $DIR/generator-tiny.rs:19:17: 19:19
24     let mut _11: u32;                    // in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
25     scope 1 {

thread '[mir-opt] mir-opt/generator-tiny.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_tiny.main-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3578:25

---- [mir-opt] mir-opt/issue-62289.rs stdout ----
---- [mir-opt] mir-opt/issue-62289.rs stdout ----
7     let mut _3: usize;                   // in scope 0 at $DIR/issue-62289.rs:9:10: 9:21
8     let mut _4: *mut u8;                 // in scope 0 at $DIR/issue-62289.rs:9:10: 9:21
9     let mut _5: std::boxed::Box<u32>;    // in scope 0 at $DIR/issue-62289.rs:9:10: 9:21
-     let mut _6: std::ops::ControlFlow<std::option::Option<std::convert::Infallible>, u32>; // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+     let mut _6: std::ops::ControlFlow<std::option::Option<!>, u32>; // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
11     let mut _7: std::option::Option<u32>; // in scope 0 at $DIR/issue-62289.rs:9:15: 9:19
12     let mut _8: isize;                   // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let _9: std::option::Option<std::convert::Infallible>; // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
+     let _9: std::option::Option<!>;      // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
14     let mut _10: !;                      // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let mut _11: std::option::Option<std::convert::Infallible>; // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
+     let mut _11: std::option::Option<!>; // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
16     let _12: u32;                        // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
17     scope 1 {

70 
71     bb5: {
71     bb5: {
72         StorageLive(_9);                 // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         _9 = ((_6 as Break).0: std::option::Option<std::convert::Infallible>); // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
+         _9 = ((_6 as Break).0: std::option::Option<!>); // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
74         StorageLive(_11);                // scope 3 at $DIR/issue-62289.rs:9:19: 9:20
75         _11 = _9;                        // scope 3 at $DIR/issue-62289.rs:9:19: 9:20
-         _0 = <Option<Box<u32>> as FromResidual<Option<Infallible>>>::from_residual(move _11) -> [return: bb6, unwind: bb12]; // scope 3 at $DIR/issue-62289.rs:9:15: 9:20
+         _0 = <Option<Box<u32>> as FromResidual<Option<!>>>::from_residual(move _11) -> [return: bb6, unwind: bb12]; // scope 3 at $DIR/issue-62289.rs:9:15: 9:20
77                                          // mir::Constant
78                                          // + span: $DIR/issue-62289.rs:9:19: 9:20
-                                          // + literal: Const { ty: fn(std::option::Option<std::convert::Infallible>) -> std::option::Option<std::boxed::Box<u32>> {<std::option::Option<std::boxed::Box<u32>> as std::ops::FromResidual<std::option::Option<std::convert::Infallible>>>::from_residual}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(std::option::Option<!>) -> std::option::Option<std::boxed::Box<u32>> {<std::option::Option<std::boxed::Box<u32>> as std::ops::FromResidual<std::option::Option<!>>>::from_residual}, val: Value(Scalar(<ZST>)) }
81 
82     bb6: {


thread '[mir-opt] mir-opt/issue-62289.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_62289.test.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/remove-never-const.rs stdout ----
---- [mir-opt] mir-opt/remove-never-const.rs stdout ----
1 // MIR for `no_codegen` after PreCodegen
3 fn no_codegen() -> () {
3 fn no_codegen() -> () {
-     let mut _0: ();                      // return place in scope 0 at $DIR/remove-never-const.rs:18:20: 18:20
+     let mut _0: ();                      // return place in scope 0 at $DIR/remove-never-const.rs:17:20: 17:20
5     scope 1 {
7 

8     bb0: {
8     bb0: {
-         unreachable;                     // scope 0 at $DIR/remove-never-const.rs:19:13: 19:33
+         unreachable;                     // scope 0 at $DIR/remove-never-const.rs:18:13: 18:33
11 }
12 


thread '[mir-opt] mir-opt/remove-never-const.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/remove_never_const.no_codegen.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/lower_intrinsics.rs stdout ----
4   fn unreachable() -> ! {
4   fn unreachable() -> ! {
5       let mut _0: !;                       // return place in scope 0 at $DIR/lower_intrinsics.rs:28:25: 28:26
6       let mut _1: !;                       // in scope 0 at $DIR/lower_intrinsics.rs:28:27: 30:2
-       let _2: ();                          // in scope 0 at $DIR/lower_intrinsics.rs:29:14: 29:45
+       let _2: !;                           // in scope 0 at $DIR/lower_intrinsics.rs:29:14: 29:45
8       let mut _3: !;                       // in scope 0 at $DIR/lower_intrinsics.rs:29:14: 29:45
9       scope 1 {


thread '[mir-opt] mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_intrinsics.unreachable.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
5       debug x => _1;                       // in scope 0 at $DIR/separate_const_switch.rs:28:13: 28:14
6       let mut _0: std::result::Result<i32, i32>; // return place in scope 0 at $DIR/separate_const_switch.rs:28:37: 28:53
7       let mut _2: i32;                     // in scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
-       let mut _3: std::ops::ControlFlow<std::result::Result<std::convert::Infallible, i32>, i32>; // in scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
+       let mut _3: std::ops::ControlFlow<std::result::Result<!, i32>, i32>; // in scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
9       let mut _4: std::result::Result<i32, i32>; // in scope 0 at $DIR/separate_const_switch.rs:29:8: 29:9
10       let mut _5: isize;                   // in scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
-       let _6: std::result::Result<std::convert::Infallible, i32>; // in scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
+       let _6: std::result::Result<!, i32>; // in scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
12       let mut _7: !;                       // in scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
-       let mut _8: std::result::Result<std::convert::Infallible, i32>; // in scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
+       let mut _8: std::result::Result<!, i32>; // in scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
14       let _9: i32;                         // in scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
15       scope 1 {
16           debug residual => _6;            // in scope 1 at $DIR/separate_const_switch.rs:29:9: 29:10
17           scope 2 {
17           scope 2 {
-               scope 8 (inlined <Result<i32, i32> as FromResidual<Result<Infallible, i32>>>::from_residual) { // at $DIR/separate_const_switch.rs:29:8: 29:10
+               scope 8 (inlined <Result<i32, i32> as FromResidual<Result<!, i32>>>::from_residual) { // at $DIR/separate_const_switch.rs:29:8: 29:10
19                   debug residual => _8;    // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
20                   let _16: i32;            // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
21                   let mut _17: i32;        // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10

40           let _11: i32;                    // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
41           let mut _12: i32;                // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
42           let _13: i32;                    // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
-           let mut _14: std::result::Result<std::convert::Infallible, i32>; // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
+           let mut _14: std::result::Result<!, i32>; // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
44           let mut _15: i32;                // in scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10
45           scope 6 {
46               debug v => _11;              // in scope 6 at $DIR/separate_const_switch.rs:29:8: 29:10
83 -     bb3: {
84 +     bb2: {
84 +     bb2: {
85           StorageLive(_6);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
-           _6 = ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>); // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
+           _6 = ((_3 as Break).0: std::result::Result<!, i32>); // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
87           StorageLive(_8);                 // scope 2 at $DIR/separate_const_switch.rs:29:9: 29:10
88           _8 = _6;                         // scope 2 at $DIR/separate_const_switch.rs:29:9: 29:10
89           StorageLive(_16);                // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10

114           ((_14 as Err).0: i32) = move _15; // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
115           discriminant(_14) = 1;           // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
116           StorageDead(_15);                // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
-           ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>) = move _14; // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
+           ((_3 as Break).0: std::result::Result<!, i32>) = move _14; // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
118           discriminant(_3) = 1;            // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
119           StorageDead(_14);                // scope 7 at $DIR/separate_const_switch.rs:29:8: 29:10
120           StorageDead(_13);                // scope 5 at $DIR/separate_const_switch.rs:29:8: 29:10

thread '[mir-opt] mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/separate_const_switch.identity.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/uninhabited-enum.rs stdout ----
---- [mir-opt] mir-opt/uninhabited-enum.rs stdout ----
1 // MIR for `process_never` after SimplifyLocals
2 
3 fn process_never(_1: *const !) -> () {
-     debug input => _1;                   // in scope 0 at $DIR/uninhabited-enum.rs:7:22: 7:27
-     let mut _0: ();                      // return place in scope 0 at $DIR/uninhabited-enum.rs:7:39: 7:39
-     let _2: &!;                          // in scope 0 at $DIR/uninhabited-enum.rs:8:8: 8:14
+     debug input => _1;                   // in scope 0 at $DIR/uninhabited-enum.rs:5:22: 5:27
+     let mut _0: ();                      // return place in scope 0 at $DIR/uninhabited-enum.rs:5:39: 5:39
+     let _2: &!;                          // in scope 0 at $DIR/uninhabited-enum.rs:6:8: 6:14
7     scope 1 {
-         debug _input => _2;              // in scope 1 at $DIR/uninhabited-enum.rs:8:8: 8:14
+         debug _input => _2;              // in scope 1 at $DIR/uninhabited-enum.rs:6:8: 6:14
10     scope 2 {
11     }

12 
12 
13     bb0: {
-         StorageLive(_2);                 // scope 0 at $DIR/uninhabited-enum.rs:8:8: 8:14
-         _2 = &(*_1);                     // scope 2 at $DIR/uninhabited-enum.rs:8:26: 8:33
-         StorageDead(_2);                 // scope 0 at $DIR/uninhabited-enum.rs:9:1: 9:2
-         unreachable;                     // scope 0 at $DIR/uninhabited-enum.rs:7:39: 9:2
+         StorageLive(_2);                 // scope 0 at $DIR/uninhabited-enum.rs:6:8: 6:14
+         _2 = &(*_1);                     // scope 2 at $DIR/uninhabited-enum.rs:6:26: 6:33
+         StorageDead(_2);                 // scope 0 at $DIR/uninhabited-enum.rs:7:1: 7:2
+         unreachable;                     // scope 0 at $DIR/uninhabited-enum.rs:5:39: 7:2
19 }
20 


thread '[mir-opt] mir-opt/uninhabited-enum.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uninhabited_enum.process_never.SimplifyLocals.after.mir', src/tools/compiletest/src/runtest.rs:3578:25

failures:
    [mir-opt] mir-opt/generator-tiny.rs
    [mir-opt] mir-opt/issue-62289.rs
---
test result: FAILED. 156 passed; 6 failed; 3 ignored; 0 measured; 0 filtered out; finished in 6.07s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:32
