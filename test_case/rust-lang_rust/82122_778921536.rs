plain
.................................................................................................... 9200/11451
.................................................................................................... 9300/11451
.................................................................................................... 9400/11451
..........i......i.................................................................................. 9500/11451
................................................iiiiiii..iiiiii.i................................... 9600/11451
.................................................................................................... 9800/11451
.................................................................................................... 9900/11451
.................................................................................................... 10000/11451
.................................................................................................... 10100/11451
---
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 154 tests
................................i...........................................iF...................... 100/154
.....................i....F........F................F.
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/inline/inline-shims.rs stdout ----
11       scope 1 {
12       }
12       }
13       scope 2 {
- +         scope 3 (inlined drop_in_place::<Option<B>> - shim(Some(Option<B>))) { // at $DIR/inline-shims.rs:12:14: 12:40
+ +         scope 3 (inlined std::ptr::drop_in_place::<Option<B>> - shim(Some(Option<B>))) { // at $DIR/inline-shims.rs:12:14: 12:40
15 +             let mut _6: isize;           // in scope 3 at $DIR/inline-shims.rs:12:14: 12:40
16 +             let mut _7: isize;           // in scope 3 at $DIR/inline-shims.rs:12:14: 12:40
17 +         }

21           StorageLive(_3);                 // scope 0 at $DIR/inline-shims.rs:11:5: 11:42
22           StorageLive(_4);                 // scope 1 at $DIR/inline-shims.rs:11:38: 11:39
23           _4 = _1;                         // scope 1 at $DIR/inline-shims.rs:11:38: 11:39
-           _3 = drop_in_place::<Vec<A>>(move _4) -> bb1; // scope 1 at $DIR/inline-shims.rs:11:14: 11:40
+           _3 = std::ptr::drop_in_place::<Vec<A>>(move _4) -> bb1; // scope 1 at $DIR/inline-shims.rs:11:14: 11:40
25                                            // mir::Constant
26                                            // + span: $DIR/inline-shims.rs:11:14: 11:37
-                                            // + literal: Const { ty: unsafe fn(*mut std::vec::Vec<A>) {std::intrinsics::drop_in_place::<std::vec::Vec<A>>}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: unsafe fn(*mut std::vec::Vec<A>) {std::ptr::drop_in_place::<std::vec::Vec<A>>}, val: Value(Scalar(<ZST>)) }
29   
30       bb1: {


32           StorageDead(_3);                 // scope 0 at $DIR/inline-shims.rs:11:41: 11:42
33           StorageLive(_5);                 // scope 2 at $DIR/inline-shims.rs:12:38: 12:39
34           _5 = _2;                         // scope 2 at $DIR/inline-shims.rs:12:38: 12:39
- -         _0 = drop_in_place::<Option<B>>(move _5) -> bb2; // scope 2 at $DIR/inline-shims.rs:12:14: 12:40
+ -         _0 = std::ptr::drop_in_place::<Option<B>>(move _5) -> bb2; // scope 2 at $DIR/inline-shims.rs:12:14: 12:40
36 -                                          // mir::Constant
37 -                                          // + span: $DIR/inline-shims.rs:12:14: 12:37
- -                                          // + literal: Const { ty: unsafe fn(*mut std::option::Option<B>) {std::intrinsics::drop_in_place::<std::option::Option<B>>}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: unsafe fn(*mut std::option::Option<B>) {std::ptr::drop_in_place::<std::option::Option<B>>}, val: Value(Scalar(<ZST>)) }
39 +         StorageLive(_6);                 // scope 2 at $DIR/inline-shims.rs:12:14: 12:40
40 +         StorageLive(_7);                 // scope 2 at $DIR/inline-shims.rs:12:14: 12:40
41 +         _6 = discriminant((*_5));        // scope 3 at $DIR/inline-shims.rs:12:14: 12:40

thread '[mir-opt] mir-opt/inline/inline-shims.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_shims.drop.Inline.diff', src/tools/compiletest/src/runtest.rs:3457:25

---- [mir-opt] mir-opt/retag.rs stdout ----
---- [mir-opt] mir-opt/retag.rs stdout ----
- // MIR for `drop_in_place` after SimplifyCfg-make_shim
+ // MIR for `std::ptr::drop_in_place` after SimplifyCfg-make_shim
2 
- fn drop_in_place(_1: *mut Test) -> () {
+ fn std::ptr::drop_in_place(_1: *mut Test) -> () {
4     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
5     let mut _2: &mut Test;               // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
6     let mut _3: ();                      // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL

thread '[mir-opt] mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.core.ptr-drop_in_place.Test.SimplifyCfg-make_shim.after.mir', src/tools/compiletest/src/runtest.rs:3457:25
---- [mir-opt] mir-opt/slice-drop-shim.rs stdout ----
---- [mir-opt] mir-opt/slice-drop-shim.rs stdout ----
- // MIR for `drop_in_place` before AddMovesForPackedDrops
+ // MIR for `std::ptr::drop_in_place` before AddMovesForPackedDrops
2 
- fn drop_in_place(_1: *mut [String]) -> () {
+ fn std::ptr::drop_in_place(_1: *mut [String]) -> () {
4     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
5     let mut _2: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
6     let mut _3: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL

thread '[mir-opt] mir-opt/slice-drop-shim.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/slice_drop_shim.core.ptr-drop_in_place.[String].AddMovesForPackedDrops.before.64bit.mir', src/tools/compiletest/src/runtest.rs:3457:25
---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
- // MIR for `drop_in_place` before AddMovesForPackedDrops
+ // MIR for `std::ptr::drop_in_place` before AddMovesForPackedDrops
2 
- fn drop_in_place(_1: *mut Vec<i32>) -> () {
+ fn std::ptr::drop_in_place(_1: *mut Vec<i32>) -> () {
4     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
5     let mut _2: &mut std::vec::Vec<i32>; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
6     let mut _3: ();                      // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL

thread '[mir-opt] mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual_item_types.core.ptr-drop_in_place.Vec_i32_.AddMovesForPackedDrops.before.64bit.mir', src/tools/compiletest/src/runtest.rs:3457:25

failures:
    [mir-opt] mir-opt/inline/inline-shims.rs
    [mir-opt] mir-opt/retag.rs
    [mir-opt] mir-opt/retag.rs
    [mir-opt] mir-opt/slice-drop-shim.rs
    [mir-opt] mir-opt/unusual-item-types.rs

test result: FAILED. 147 passed; 4 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.24s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:32
