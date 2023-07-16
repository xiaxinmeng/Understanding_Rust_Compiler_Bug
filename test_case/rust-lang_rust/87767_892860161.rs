plain
failures:

---- [codegen] codegen/slice-windows-no-bounds-check.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-10/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll" "/checkout/src/test/codegen/slice-windows-no-bounds-check.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/slice-windows-no-bounds-check.rs:13:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll:35:108: note: found here
 tail call void @_ZN4core5slice5index26slice_start_index_len_fail17hb2087d6e663b54d2E(i64 1, i64 0, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc31 to %"std::panic::Location"*)), !noalias !2
                                                                                                           ^~~~~
/checkout/src/test/codegen/slice-windows-no-bounds-check.rs:24:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll:78:108: note: found here
 tail call void @_ZN4core5slice5index26slice_start_index_len_fail17hb2087d6e663b54d2E(i64 1, i64 0, %"std::panic::Location"* noalias nonnull readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc31 to %"std::panic::Location"*))
                                                                                                           ^~~~~
/checkout/src/test/codegen/slice-windows-no-bounds-check.rs:32:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll:114:111: note: found here
 tail call void @_ZN4core5slice5index24slice_end_index_len_fail17h7f06cac9bbfc3f03E(i64 %_20.i, i64 0, %"std::panic::Location"* noalias nonnull readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc10 to %"std::panic::Location"*))

------------------------------------------


---
test result: FAILED. 220 passed; 1 failed; 59 ignored; 0 measured; 0 filtered out; finished in 2.90s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:18
