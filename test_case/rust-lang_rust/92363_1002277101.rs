plain
Warning: Skipping "src/test/mir-opt": not a regular file or directory
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 166 tests
...................F...............i.........F...................i...............i.................. 100/166
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
.......F........................i.................................

---- [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
---- [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
1 - // MIR for `main` before ConstProp
2 + // MIR for `main` after ConstProp
+   
4   fn main() -> () {
4   fn main() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:4:11: 4:11
6       let _1: *const [i32];                // in scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:9: 5:10
20               }
21           }
22       }
- 
- 
+   
24       bb0: {
25           StorageLive(_1);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:9: 5:10
26           StorageLive(_2);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35

46 +         _8 = Lt(const 3_usize, _7);      // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:18: 7:25
47 +         assert(move _8, "index out of bounds: the length is {} but the index is {}", move _7, const 3_usize) -> bb1; // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:18: 7:25
- 
+   
50       bb1: {
50       bb1: {
51           _5 = (*_1)[_6];                  // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:18: 7:25
52           StorageDead(_6);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:25: 7:26

56           return;                          // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:9:2: 9:2
58   }
- 
+   
60 
60 

thread '[mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3360:25

---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
1 - // MIR for `main` before ConstProp
2 + // MIR for `main` after ConstProp
+   
4   fn main() -> () {
4   fn main() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/slice_len.rs:4:11: 4:11
6       let _1: u32;                         // in scope 0 at $DIR/slice_len.rs:5:5: 5:33

13       let mut _8: bool;                    // in scope 0 at $DIR/slice_len.rs:5:5: 5:33
14       let mut _9: &[u32; 3];               // in scope 0 at $DIR/slice_len.rs:5:6: 5:19
15       let mut _10: &[u32; 3];              // in scope 0 at $DIR/slice_len.rs:5:6: 5:19
+   
17       bb0: {
17       bb0: {
18           StorageLive(_1);                 // scope 0 at $DIR/slice_len.rs:5:5: 5:33
19           StorageLive(_2);                 // scope 0 at $DIR/slice_len.rs:5:5: 5:30

41 +         _8 = const true;                 // scope 0 at $DIR/slice_len.rs:5:5: 5:33
42 +         assert(const true, "index out of bounds: the length is {} but the index is {}", const 3_usize, const 1_usize) -> bb1; // scope 0 at $DIR/slice_len.rs:5:5: 5:33
- 
+   
45       bb1: {
45       bb1: {
46 -         _1 = (*_2)[_6];                  // scope 0 at $DIR/slice_len.rs:5:5: 5:33
47 +         _1 = const 2_u32;                // scope 0 at $DIR/slice_len.rs:5:5: 5:33

53           return;                          // scope 0 at $DIR/slice_len.rs:6:2: 6:2
55   }
- 
+   
57 
57 

thread '[mir-opt] mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/slice_len.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3360:25
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
1 - // MIR for `main` before SimplifyArmIdentity
2 + // MIR for `main` after SimplifyArmIdentity
+   
4   fn main() -> () {
4   fn main() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/issue-73223.rs:1:11: 1:11
6       let _1: i32;                         // in scope 0 at $DIR/issue-73223.rs:2:9: 2:14
47       scope 2 {
47       scope 2 {
48           debug v => _4;                   // in scope 2 at $DIR/issue-73223.rs:3:14: 3:15
- 
+   
51       bb0: {
51       bb0: {
52           StorageLive(_1);                 // scope 0 at $DIR/issue-73223.rs:2:9: 2:14
53           StorageLive(_2);                 // scope 0 at $DIR/issue-73223.rs:2:23: 2:30

56           _3 = const 1_isize;              // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
57           goto -> bb2;                     // scope 0 at $DIR/issue-73223.rs:2:17: 2:30
- 
+   
60       bb1: {
60       bb1: {
61           nop;                             // scope 0 at $DIR/issue-73223.rs:4:17: 4:23
62           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:5:6: 5:7

63           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
64           return;                          // scope 0 at $DIR/issue-73223.rs:9:2: 9:2
- 
+   
67       bb2: {
67       bb2: {
68           StorageLive(_4);                 // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
69           _4 = ((_2 as Some).0: i32);      // scope 0 at $DIR/issue-73223.rs:3:14: 3:15

110           StorageDead(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
111           switchInt(move _15) -> [false: bb4, otherwise: bb3]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- 
+   
114       bb3: {
114       bb3: {
115           StorageLive(_20);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
116           discriminant(_20) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

144                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
145                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }
- 
+   
148       bb4: {
148       bb4: {
149           nop;                             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
150           StorageDead(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

158           return;                          // scope 0 at $DIR/issue-73223.rs:9:2: 9:2
160   }
- 
+   
162 
162 

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3360:25

failures:
    [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
    [mir-opt] mir-opt/const_prop/slice_len.rs
    [mir-opt] mir-opt/const_prop/slice_len.rs
    [mir-opt] mir-opt/issue-73223.rs

test result: FAILED. 159 passed; 3 failed; 4 ignored; 0 measured; 0 filtered out; finished in 3.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:33
