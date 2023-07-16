plain
.................................................................................................... 9200/11472
.................................................................................................... 9300/11472
.................................................................................................... 9400/11472
.............................i......i............................................................... 9500/11472
....................................................................iiiiiii..iiiiii.i............... 9600/11472
.................................................................................................... 9800/11472
.................................................................................................... 9900/11472
.................................................................................................... 10000/11472
.................................................................................................... 10100/11472
---
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...............F.....i................................
failures:

---- [mir-opt] mir-opt/lower_intrinsics.rs stdout ----
30           _4 = _1;                         // scope 0 at $DIR/lower_intrinsics.rs:7:45: 7:46
31           StorageLive(_5);                 // scope 0 at $DIR/lower_intrinsics.rs:7:48: 7:49
32           _5 = _2;                         // scope 0 at $DIR/lower_intrinsics.rs:7:48: 7:49
- -         _3 = wrapping_add::<T>(move _4, move _5) -> bb1; // scope 0 at $DIR/lower_intrinsics.rs:7:14: 7:50
+ -         _3 = std::intrinsics::wrapping_add::<T>(move _4, move _5) -> bb1; // scope 0 at $DIR/lower_intrinsics.rs:7:14: 7:50
34 -                                          // mir::Constant
35 -                                          // + span: $DIR/lower_intrinsics.rs:7:14: 7:44
36 -                                          // + literal: Const { ty: extern "rust-intrinsic" fn(T, T) -> T {std::intrinsics::wrapping_add::<T>}, val: Value(Scalar(<ZST>)) }

thread '[mir-opt] mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_intrinsics.wrapping.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3457:25


failures:
    [mir-opt] mir-opt/lower_intrinsics.rs
    [mir-opt] mir-opt/lower_intrinsics.rs

test result: FAILED. 150 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 4.18s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:58
