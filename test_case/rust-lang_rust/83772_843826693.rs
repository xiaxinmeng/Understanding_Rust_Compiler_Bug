plain
.................................................................................................... 9500/11891
.................................................................................................... 9600/11891
...................................................................................i.......i........ 9700/11891
.................................................................................................... 9800/11891
............................iiiiiii..iiiiii.i....................................................... 9900/11891
.................................................................................................... 10100/11891
.................................................................................................... 10200/11891
.................................................................................................... 10300/11891
.................................................................................................... 10400/11891
---
.........................F.i................................
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/remove_storage_markers.rs stdout ----
67           _9 = &mut (*_10);                // scope 3 at $DIR/remove_storage_markers.rs:8:14: 8:19
68 -         StorageLive(_18);                // scope 7 at $DIR/remove_storage_markers.rs:8:14: 8:19
69           _18 = &mut (*_9);                // scope 7 at $DIR/remove_storage_markers.rs:8:14: 8:19
-           _8 = <std::ops::Range<i32> as iter::range::RangeIteratorImpl>::specialized_next(move _18) -> bb4; // scope 7 at $DIR/remove_storage_markers.rs:8:14: 8:19
+           _8 = <std::ops::Range<i32> as iter::range::RangeIteratorImpl>::spec_next(move _18) -> bb4; // scope 7 at $DIR/remove_storage_markers.rs:8:14: 8:19
71                                            // mir::Constant
72                                            // + span: $DIR/remove_storage_markers.rs:8:14: 8:19
-                                            // + literal: Const { ty: for<'r> fn(&'r mut std::ops::Range<i32>) -> std::option::Option<<std::ops::Range<i32> as std::iter::range::RangeIteratorImpl>::Item> {<std::ops::Range<i32> as std::iter::range::RangeIteratorImpl>::specialized_next}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r> fn(&'r mut std::ops::Range<i32>) -> std::option::Option<<std::ops::Range<i32> as std::iter::range::RangeIteratorImpl>::Item> {<std::ops::Range<i32> as std::iter::range::RangeIteratorImpl>::spec_next}, val: Value(Scalar(<ZST>)) }
75   
76       bb2: {


thread '[mir-opt] mir-opt/remove_storage_markers.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/remove_storage_markers.main.RemoveStorageMarkers.diff', src/tools/compiletest/src/runtest.rs:3554:25


failures:
    [mir-opt] mir-opt/remove_storage_markers.rs
    [mir-opt] mir-opt/remove_storage_markers.rs

test result: FAILED. 156 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:36
