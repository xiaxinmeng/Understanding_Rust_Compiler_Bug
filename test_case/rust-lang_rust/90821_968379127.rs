plain
failures:

---- [codegen] codegen/slice-reverse.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-reverse/slice-reverse.ll" "/checkout/src/test/codegen/slice-reverse.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/slice-reverse.rs:11:12: error: CHECK: expected string not found in input
 // CHECK: shufflevector <{{[0-9]+}} x i8>
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-reverse/slice-reverse.ll:15:30: note: scanning from here
define void @slice_reverse_u8([0 x i8]* noalias nonnull align 1 %slice.0, i64 %slice.1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                             ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-reverse/slice-reverse.ll:19:8: note: possible intended match here
 %0 = getelementptr inbounds [0 x i8], [0 x i8]* %slice.0, i64 0, i64 %slice.1
       ^
/checkout/src/test/codegen/slice-reverse.rs:22:12: error: CHECK: expected string not found in input
 // CHECK: shufflevector <{{[0-9]+}} x i32>
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-reverse/slice-reverse.ll:59:31: note: scanning from here
define void @slice_reverse_i32([0 x i32]* noalias nonnull align 4 %slice.0, i64 %slice.1) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-reverse/slice-reverse.ll:63:8: note: possible intended match here
 %0 = getelementptr inbounds [0 x i32], [0 x i32]* %slice.0, i64 0, i64 %slice.1

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-reverse/slice-reverse.ll
Check file: /checkout/src/test/codegen/slice-reverse.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           10: @alloc43 = private unnamed_addr constant <{ [64 x i8] }> <{ [64 x i8] c"attempt to create slice covering at least half the address space" }>, align 1
           11: @alloc45 = private unnamed_addr constant <{ [39 x i8] }> <{ [39 x i8] c"/checkout/library/core/src/slice/raw.rs" }>, align 1
           12: @alloc42 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [39 x i8] }>, <{ [39 x i8] }>* @alloc45, i32 0, i32 0, i32 0), [16 x i8] c"'\00\00\00\00\00\00\00\9E\00\00\00\05\00\00\00" }>, align 8
           13: 
           14: ; Function Attrs: nounwind nonlazybind uwtable
           15: define void @slice_reverse_u8([0 x i8]* noalias nonnull align 1 %slice.0, i64 %slice.1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:11'0                                  X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           16: start:
check:11'0     ~~~~~~
           17:  tail call void @llvm.experimental.noalias.scope.decl(metadata !2)
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  %half_len.i = lshr i64 %slice.1, 1
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19:  %0 = getelementptr inbounds [0 x i8], [0 x i8]* %slice.0, i64 0, i64 %slice.1
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:11'1            ?                                                                       possible intended match
           20:  tail call void @llvm.experimental.noalias.scope.decl(metadata !5), !noalias !2
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21:  tail call void @llvm.experimental.noalias.scope.decl(metadata !8), !noalias !2
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22:  %.not.i.i = icmp eq i64 %half_len.i, 0
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           23:  br i1 %.not.i.i, label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse17h89e02073719f415aE.exit", label %bb16.i.i
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24: 
check:11'0     ~
            .
            .
            .
           54: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse17h89e02073719f415aE.exit": ; preds = %_ZN4core3mem4swap17h5fa626842d36eb7eE.exit.i.i, %start
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55:  ret void
check:11'0     ~~~~~~~~~
           56: }
check:11'0     ~
           57: 
check:11'0     ~
           58: ; Function Attrs: nonlazybind uwtable
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59: define void @slice_reverse_i32([0 x i32]* noalias nonnull align 4 %slice.0, i64 %slice.1) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:22'0                                   X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           60: start:
check:22'0     ~~~~~~
           61:  tail call void @llvm.experimental.noalias.scope.decl(metadata !22)
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62:  %half_len.i = lshr i64 %slice.1, 1
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  %0 = getelementptr inbounds [0 x i32], [0 x i32]* %slice.0, i64 0, i64 %slice.1
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:22'1            ?                                                                         possible intended match
           64:  %1 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %half_len.i, i64 4) #6
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65:  %2 = extractvalue { i64, i1 } %1, 0
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66:  %3 = extractvalue { i64, i1 } %1, 1
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67:  %_72.i.i.i = icmp slt i64 %2, 0
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68:  %_7.i.i.i = or i1 %3, %_72.i.i.i
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>

---
test result: FAILED. 253 passed; 1 failed; 47 ignored; 0 measured; 0 filtered out; finished in 3.64s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:47
