plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.............F..................i.................................
failures:

---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
4   static mut BAR: *const &i32 = {
5       let mut _0: *const &i32;             // return place in scope 0 at $DIR/const-promotion-extern-static.rs:9:17: 9:28
6       let mut _1: &[&i32];                 // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
-       let mut _2: &[&i32; 1];              // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
-       let _3: [&i32; 1];                   // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
-       let mut _4: &i32;                    // in scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
-       let _5: &i32;                        // in scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
- +     let mut _6: &[&i32; 1];              // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+       let _2: &[&i32];                     // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+       let mut _3: &[&i32; 1];              // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+       let _4: [&i32; 1];                   // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
+       let mut _5: &i32;                    // in scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
+       let _6: &i32;                        // in scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
+ +     let mut _7: &[&i32; 1];              // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
13       bb0: {
13       bb0: {
14           StorageLive(_1);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44

15           StorageLive(_2);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
- -         StorageLive(_3);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
- -         StorageLive(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
- -         StorageLive(_5);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
- -         _5 = const {alloc1: &i32};       // scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
- +         _6 = const BAR::promoted[0];     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+           StorageLive(_3);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+ -         StorageLive(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
+ -         StorageLive(_5);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
+ -         StorageLive(_6);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
+ -         _6 = const {alloc1: &i32};       // scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
+ +         _7 = const BAR::promoted[0];     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
21                                            // ty::Const
22 -                                          // + ty: &i32
23 -                                          // + val: Value(Scalar(alloc1))
26                                            // mir::Constant
26                                            // mir::Constant
27 -                                          // + span: $DIR/const-promotion-extern-static.rs:9:33: 9:34
28 -                                          // + literal: Const { ty: &i32, val: Value(Scalar(alloc1)) }
- -         _4 = &(*_5);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
- -         _3 = [move _4];                  // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
- -         _2 = &_3;                        // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+ -         _5 = &(*_6);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
+ -         _4 = [move _5];                  // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
+ -         _3 = &_4;                        // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
32 +                                          // + span: $DIR/const-promotion-extern-static.rs:9:31: 9:44
33 +                                          // + literal: Const { ty: &[&i32; 1], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ const_promotion_extern_static[HASH]::BAR), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
- +         _2 = &(*_6);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
-           _1 = move _2 as &[&i32] (Pointer(Unsize)); // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
- -         StorageDead(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35
-           StorageDead(_2);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35
+ +         _3 = &(*_7);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+           _2 = move _3 as &[&i32] (Pointer(Unsize)); // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+           _1 = &(*_2);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+ -         StorageDead(_5);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35
+           StorageDead(_3);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35
38           _0 = core::slice::<impl [&i32]>::as_ptr(move _1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
39                                            // mir::Constant
40                                            // + span: $DIR/const-promotion-extern-static.rs:9:36: 9:42
42       }
43   
44       bb1: {
44       bb1: {
- -         StorageDead(_5);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
- -         StorageDead(_3);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
+ -         StorageDead(_6);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
+ -         StorageDead(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
+           StorageDead(_2);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
47           StorageDead(_1);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
48           return;                          // scope 0 at $DIR/const-promotion-extern-static.rs:9:1: 9:45


thread '[mir-opt] mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3360:25

---- [mir-opt] mir-opt/lower_array_len.rs stdout ----
---- [mir-opt] mir-opt/lower_array_len.rs stdout ----
9       let mut _4: usize;                   // in scope 0 at $DIR/lower_array_len.rs:7:8: 7:13
10       let mut _5: usize;                   // in scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
11       let mut _6: &[u8];                   // in scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
-       let mut _7: &[u8; N];                // in scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
-       let _8: usize;                       // in scope 0 at $DIR/lower_array_len.rs:8:15: 8:20
-       let mut _9: usize;                   // in scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
-       let mut _10: bool;                   // in scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
- +     let mut _11: &[u8; N];               // in scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
+       let _7: &[u8];                       // in scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
+       let mut _8: &[u8; N];                // in scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
+       let _9: usize;                       // in scope 0 at $DIR/lower_array_len.rs:8:15: 8:20
+       let mut _10: usize;                  // in scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
+       let mut _11: bool;                   // in scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
18       bb0: {
18       bb0: {
19           StorageLive(_3);                 // scope 0 at $DIR/lower_array_len.rs:7:8: 7:27

22           StorageLive(_5);                 // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
23           StorageLive(_6);                 // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
24           StorageLive(_7);                 // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
-           _7 = &(*_2);                     // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
- +         StorageLive(_11);                // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
- +         _11 = _7;                        // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
-           _6 = move _7 as &[u8] (Pointer(Unsize)); // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
-           StorageDead(_7);                 // scope 0 at $DIR/lower_array_len.rs:7:20: 7:21
- -         _5 = Len((*_6));                 // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
- +         _5 = Len((*_11));                // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
- +         StorageDead(_11);                // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
+           StorageLive(_8);                 // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
+           _8 = &(*_2);                     // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
+           _7 = move _8 as &[u8] (Pointer(Unsize)); // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
+           _6 = &(*_7);                     // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
+           StorageDead(_8);                 // scope 0 at $DIR/lower_array_len.rs:7:20: 7:21
+           _5 = Len((*_6));                 // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
33           goto -> bb1;                     // scope 0 at $DIR/lower_array_len.rs:7:16: 7:27
35   

36       bb1: {
36       bb1: {
37           StorageDead(_6);                 // scope 0 at $DIR/lower_array_len.rs:7:26: 7:27
38           _3 = Lt(move _4, move _5);       // scope 0 at $DIR/lower_array_len.rs:7:8: 7:27
+           StorageDead(_7);                 // scope 0 at $DIR/lower_array_len.rs:7:26: 7:27
39           StorageDead(_5);                 // scope 0 at $DIR/lower_array_len.rs:7:26: 7:27
40           StorageDead(_4);                 // scope 0 at $DIR/lower_array_len.rs:7:26: 7:27
41           switchInt(move _3) -> [false: bb4, otherwise: bb2]; // scope 0 at $DIR/lower_array_len.rs:7:8: 7:27
42       }
43   
44       bb2: {
44       bb2: {
-           StorageLive(_8);                 // scope 0 at $DIR/lower_array_len.rs:8:15: 8:20
-           _8 = _1;                         // scope 0 at $DIR/lower_array_len.rs:8:15: 8:20
-           _9 = Len((*_2));                 // scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
-           _10 = Lt(_8, _9);                // scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
-           assert(move _10, "index out of bounds: the length is {} but the index is {}", move _9, _8) -> bb3; // scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
+           StorageLive(_9);                 // scope 0 at $DIR/lower_array_len.rs:8:15: 8:20
+           _9 = _1;                         // scope 0 at $DIR/lower_array_len.rs:8:15: 8:20
+           _10 = Len((*_2));                // scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
+           _11 = Lt(_9, _10);               // scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
+           assert(move _11, "index out of bounds: the length is {} but the index is {}", move _10, _9) -> bb3; // scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
51   
52       bb3: {


-           _0 = (*_2)[_8];                  // scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
-           StorageDead(_8);                 // scope 0 at $DIR/lower_array_len.rs:9:5: 9:6
+           _0 = (*_2)[_9];                  // scope 0 at $DIR/lower_array_len.rs:8:9: 8:21
+           StorageDead(_9);                 // scope 0 at $DIR/lower_array_len.rs:9:5: 9:6
55           goto -> bb5;                     // scope 0 at $DIR/lower_array_len.rs:7:5: 11:6
57   


thread '[mir-opt] mir-opt/lower_array_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_array_len.array_bound.NormalizeArrayLen.diff', src/tools/compiletest/src/runtest.rs:3360:25

failures:
    [mir-opt] mir-opt/const-promotion-extern-static.rs
    [mir-opt] mir-opt/lower_array_len.rs
    [mir-opt] mir-opt/lower_array_len.rs

test result: FAILED. 161 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.88s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:59
