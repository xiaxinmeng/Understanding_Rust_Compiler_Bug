plain
Successfully built ae9686c646fa
Successfully tagged rust-ci:latest
Built container sha256:ae9686c646fa0a8e2d99785807b0ba0002702552509d4ea868a8f2a5367883c4
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] codegen/unwind-and-panic-abort.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-and-panic-abort/unwind-and-panic-abort.ll" "/checkout/src/test/codegen/unwind-and-panic-abort.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/unwind-and-panic-abort.rs:12:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.trap()
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-and-panic-abort/unwind-and-panic-abort.ll:14:17: note: scanning from here
define void @foo() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-and-panic-abort/unwind-and-panic-abort.ll:26:7: note: possible intended match here
 tail call void @_ZN4core9panicking15panic_no_unwind17h8ea4754236b242e2E(%"core::panic::location::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc19 to %"core::panic::location::Location"*)) #3

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-and-panic-abort/unwind-and-panic-abort.ll
Check file: /checkout/src/test/codegen/unwind-and-panic-abort.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            9: 
           10: @alloc18 = private unnamed_addr constant <{ [52 x i8] }> <{ [52 x i8] c"/checkout/src/test/codegen/unwind-and-panic-abort.rs" }>, align 1
           11: @alloc19 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [52 x i8] }>, <{ [52 x i8] }>* @alloc18, i32 0, i32 0, i32 0), [16 x i8] c"4\00\00\00\00\00\00\00\0E\00\00\00\01\00\00\00" }>, align 8
           12: 
           13: ; Function Attrs: nounwind nonlazybind
           14: define void @foo() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:12'0                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           15: start:
check:12'0     ~~~~~~
           16:  invoke void @bar()
check:12'0     ~~~~~~~~~~~~~~~~~~~
           17:  to label %bb1 unwind label %cleanup
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: 
check:12'0     ~
           19: bb1: ; preds = %start
check:12'0     ~~~~~~~~~~~~~~~~~~~~~
           20:  ret void
check:12'0     ~~~~~~~~~
           21: 
check:12'0     ~
           22: cleanup: ; preds = %start
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           23:  %0 = landingpad { i8*, i32 }
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24:  cleanup
check:12'0     ~~~~~~~~
           25: ; call core::panicking::panic_no_unwind
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  tail call void @_ZN4core9panicking15panic_no_unwind17h8ea4754236b242e2E(%"core::panic::location::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc19 to %"core::panic::location::Location"*)) #3
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:12'1           ?                                                                                                                                                                                                                                       possible intended match
           27:  unreachable
check:12'0     ~~~~~~~~~~~~
           28: }
check:12'0     ~
           29: 
check:12'0     ~
           30: ; Function Attrs: nounwind nonlazybind
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31: declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #0
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>

---
test result: FAILED. 262 passed; 1 failed; 51 ignored; 0 measured; 0 filtered out; finished in 3.84s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:32
