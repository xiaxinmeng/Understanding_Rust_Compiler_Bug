plain
..............................i..........................F......
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/uniform_array_move_out.rs stdout ----
30         StorageLive(_2);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:14: 5:19
31         _3 = SizeOf(i32);                // scope 2 at $DIR/uniform_array_move_out.rs:5:14: 5:19
32         _4 = AlignOf(i32);               // scope 2 at $DIR/uniform_array_move_out.rs:5:14: 5:19
-         _5 = alloc::alloc::exchange_malloc(move _3, move _4) -> [return: bb1, unwind: bb12]; // scope 2 at $DIR/uniform_array_move_out.rs:5:14: 5:19
+         _5 = alloc::alloc::exchange_malloc(move _3, move _4) -> bb1; // scope 2 at $DIR/uniform_array_move_out.rs:5:14: 5:19
34                                          // mir::Constant
35                                          // + span: $DIR/uniform_array_move_out.rs:5:14: 5:19
36                                          // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(<ZST>)) }

49         StorageLive(_7);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:21: 5:26
50         _8 = SizeOf(i32);                // scope 3 at $DIR/uniform_array_move_out.rs:5:21: 5:26
51         _9 = AlignOf(i32);               // scope 3 at $DIR/uniform_array_move_out.rs:5:21: 5:26
-         _10 = alloc::alloc::exchange_malloc(move _8, move _9) -> [return: bb3, unwind: bb11]; // scope 3 at $DIR/uniform_array_move_out.rs:5:21: 5:26
+         _10 = alloc::alloc::exchange_malloc(move _8, move _9) -> bb3; // scope 3 at $DIR/uniform_array_move_out.rs:5:21: 5:26
53                                          // mir::Constant
54                                          // + span: $DIR/uniform_array_move_out.rs:5:21: 5:26
55                                          // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(<ZST>)) }

thread '[mir-opt] mir-opt/uniform_array_move_out.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uniform_array_move_out.move_out_from_end.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3578:25


failures:
    [mir-opt] mir-opt/uniform_array_move_out.rs
    [mir-opt] mir-opt/uniform_array_move_out.rs

test result: FAILED. 160 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.26s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:46
