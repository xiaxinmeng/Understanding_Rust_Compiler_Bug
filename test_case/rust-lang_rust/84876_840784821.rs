plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu

failures:

---- [codegen] codegen/thread-local.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll" "/checkout/src/test/codegen/thread-local.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/thread-local.rs:36:12: error: CHECK: expected string not found in input
 // CHECK: %0 = load i64, i64* [[TLS_AUX]], align 8
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:27:20: note: scanning from here
define i64 @get_aux() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:27:20: note: with "TLS_AUX" equal to "@_ZN16thread_local_aux1A7__getit3VAL17hab8dc2f9a80464fcE"
define i64 @get_aux() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:29:2: note: possible intended match here
 %0 = load i64, i64* @_ZN16thread_local_aux1A7__getit3VAL17hab8dc2f9a80464fcE, align 4
 ^
/checkout/src/test/codegen/thread-local.rs:44:12: error: CHECK: expected string not found in input
 // CHECK: store i64 %0, i64* [[TLS_AUX]], align 8
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:34:21: note: scanning from here
define void @set_aux(i64 %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                    ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:34:21: note: with "TLS_AUX" equal to "@_ZN16thread_local_aux1A7__getit3VAL17hab8dc2f9a80464fcE"
define void @set_aux(i64 %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                    ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:36:2: note: possible intended match here
 store i64 %0, i64* @_ZN16thread_local_aux1A7__getit3VAL17hab8dc2f9a80464fcE, align 4, !alias.scope !12, !noalias !15

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll
Check file: /checkout/src/test/codegen/thread-local.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           22:  store i32 %0, i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h91372f507ed0ea49E to i32*), align 4, !alias.scope !2, !noalias !5
           23:  ret void
           24: }
           25: 
           26: ; Function Attrs: norecurse nounwind nonlazybind readonly uwtable willreturn
           27: define i64 @get_aux() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:36'0                        X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:36'1                                                                                                                                                                                     with "TLS_AUX" equal to "@_ZN16thread_local_aux1A7__getit3VAL17hab8dc2f9a80464fcE"
           28: start:
check:36'0     ~~~~~~
           29:  %0 = load i64, i64* @_ZN16thread_local_aux1A7__getit3VAL17hab8dc2f9a80464fcE, align 4
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:36'2      ?                                                                                     possible intended match
           30:  ret i64 %0
check:36'0     ~~~~~~~~~~~
           31: }
check:36'0     ~
           32: 
check:36'0     ~
           33: ; Function Attrs: nofree nounwind nonlazybind uwtable willreturn
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           34: define void @set_aux(i64 %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:36'0     ~~~~~~~~~~~~~~~~~~~~
check:44'0                         X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:44'1                                                                                                                                                                                            with "TLS_AUX" equal to "@_ZN16thread_local_aux1A7__getit3VAL17hab8dc2f9a80464fcE"
           35: start:
check:44'0     ~~~~~~
           36:  store i64 %0, i64* @_ZN16thread_local_aux1A7__getit3VAL17hab8dc2f9a80464fcE, align 4, !alias.scope !12, !noalias !15
check:44'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:44'2      ?                                                                                                                    possible intended match
           37:  ret void
check:44'0     ~~~~~~~~~
           38: }
check:44'0     ~
           39: 
check:44'0     ~
           40: ; Function Attrs: nounwind nonlazybind uwtable
check:44'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41: declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #2
check:44'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>

---
test result: FAILED. 218 passed; 1 failed; 53 ignored; 0 measured; 0 filtered out; finished in 3.22s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-i586-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target i586-unknown-linux-gnu,i686-unknown-linux-musl
Build completed unsuccessfully in 0:21:29
