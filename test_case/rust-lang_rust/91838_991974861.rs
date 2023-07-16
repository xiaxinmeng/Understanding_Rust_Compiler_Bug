plain
failures:

---- [codegen] codegen/slice-ref-equality.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll" "/checkout/src/test/codegen/slice-ref-equality.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/slice-ref-equality.rs:15:17: error: CHECK-NEXT: expected string not found in input
 // CHECK-NEXT: %{{.+}} = getelementptr {{.+}}
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:9:2: note: scanning from here
 %0 = bitcast [4 x i8]* %data to i32*
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:11:2: note: possible intended match here
 %2 = icmp eq i32 %1, 0

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll
Check file: /checkout/src/test/codegen/slice-ref-equality.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'slice_ref_equality.4ddab54c-cgu.0'
           2: source_filename = "slice_ref_equality.4ddab54c-cgu.0"
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
           4: target triple = "x86_64-unknown-linux-gnu"
           5: 
           6: ; Function Attrs: nofree nounwind nonlazybind uwtable willreturn
           7: define zeroext i1 @is_zero_slice([4 x i8]* noalias readonly align 1 dereferenceable(4) %data) unnamed_addr #0 {
           8: "_ZN4core5array8equality96_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$A$u3b$$u20$N$u5d$$GT$$u20$for$u20$$RF$$u5b$B$u5d$$GT$2eq17h31958599dafb137dE.exit":
           9:  %0 = bitcast [4 x i8]* %data to i32*
next:15'0      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          10:  %1 = load i32, i32* %0, align 1, !alias.scope !2
next:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          11:  %2 = icmp eq i32 %1, 0
next:15'0     ~~~~~~~~~~~~~~~~~~~~~~~
next:15'1      ?                      possible intended match
          12:  ret i1 %2
next:15'0     ~~~~~~~~~~
          13: }
next:15'0     ~
          14: 
next:15'0     ~
          15: ; Function Attrs: nofree nounwind nonlazybind uwtable willreturn
next:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          16: define zeroext i1 @is_zero_array([4 x i8]* noalias nocapture readonly align 1 dereferenceable(4) %data) unnamed_addr #0 {
next:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>

---
test result: FAILED. 256 passed; 1 failed; 49 ignored; 0 measured; 0 filtered out; finished in 3.59s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:49
