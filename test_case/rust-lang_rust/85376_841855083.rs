plain
.................................................................................................... 9500/11877
.................................................................................................... 9600/11877
..............................................................................i......i.............. 9700/11877
.................................................................................................... 9800/11877
.......................iiiiiii..iiiiii.i............................................................ 9900/11877
.................................................................................................... 10100/11877
.................................................................................................... 10200/11877
.................................................................................................... 10300/11877
.................................................................................................... 10400/11877
---
 finished in 0.415 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 36 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 32 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.177 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.39s

 finished in 3.458 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 25 tests
iiiiiiiiiiiii............

 finished in 2.504 seconds
Build completed successfully in 0:27:47
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Warning: Skipping "src/test/mir-opt": not a regular file or directory
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 160 tests
..................................i..F..........................i..............i.......F............ 100/160
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
29 +         _2 = const (4_i32, false);       // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
30 +                                          // mir::Constant
31 +                                          // + span: $DIR/optimizes_into_variable.rs:12:13: 12:18
- +                                          // + literal: Const { ty: (i32, bool), val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [31], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: (i32, bool), val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [31], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
33 +         assert(!const false, "attempt to compute `{} + {}`, which would overflow", const 2_i32, const 2_i32) -> bb1; // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
35   


thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/optimizes_into_variable.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3554:25

---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
23 +         ((*_4).0: alloc::raw_vec::RawVec<u32>) = const alloc::raw_vec::RawVec::<u32> { ptr: Unique::<u32> { pointer: {0x4 as *const u32}, _marker: PhantomData::<u32> }, cap: 0_usize, alloc: std::alloc::Global }; // scope 2 at $DIR/inline-into-box-place.rs:8:33: 8:43
24 +                                          // ty::Const
25 +                                          // + ty: alloc::raw_vec::RawVec<u32>
- +                                          // + val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } })
+ +                                          // + val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } })
27                                            // mir::Constant
28 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
29 -                                          // + user_ty: UserType(1)
33 -     bb1: {
33 -     bb1: {
34 +                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:43
35 +                                          // + user_ty: UserType(0)
- +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
37 +         ((*_4).1: usize) = const 0_usize; // scope 2 at $DIR/inline-into-box-place.rs:8:33: 8:43
38 +         StorageDead(_4);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
39           _1 = move _2;                    // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43

thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.32bit.diff', src/tools/compiletest/src/runtest.rs:3554:25

failures:
    [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs
    [mir-opt] mir-opt/inline/inline-into-box-place.rs
    [mir-opt] mir-opt/inline/inline-into-box-place.rs

test result: FAILED. 154 passed; 2 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.01s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:22
