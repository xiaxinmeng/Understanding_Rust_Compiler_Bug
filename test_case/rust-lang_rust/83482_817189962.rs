plain
failures:

---- [codegen] codegen/force-no-unwind-tables.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/force-no-unwind-tables/force-no-unwind-tables.ll" "/checkout/src/test/codegen/force-no-unwind-tables.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/force-no-unwind-tables.rs:6:17: error: CHECK-LABEL: expected string not found in input
// CHECK-LABEL: define void @foo
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/force-no-unwind-tables/force-no-unwind-tables.ll:1:1: note: scanning from here
; ModuleID = 'force_no_unwind_tables.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/force-no-unwind-tables/force-no-unwind-tables.ll:55:10: note: possible intended match here
define internal void @_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h317267e4903dea04E(%"[closure@std::rt::begin_panic<&str>::{closure#0}]"* noalias nocapture dereferenceable(12) %f) unnamed_addr #2 personality i32 (...)* @rust_eh_personality {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/force-no-unwind-tables/force-no-unwind-tables.ll
Check file: /checkout/src/test/codegen/force-no-unwind-tables.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'force_no_unwind_tables.3a1fbbbh-cgu.0'
label:6'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "force_no_unwind_tables.3a1fbbbh-cgu.0"
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "wasm32-unknown-emscripten"
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5: 
label:6'0     ~
           6: %"[closure@std::rt::begin_panic<&str>::{closure#0}]" = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], %"std::panic::Location"*, [0 x i32] }
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
          50:  ret i64 %0
          50:  ret i64 %0
label:6'0     ~~~~~~~~~~~
          51: }
label:6'0     ~
          52: 
label:6'0     ~
          53: ; std::sys_common::backtrace::__rust_end_short_backtrace
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          54: ; Function Attrs: noinline noreturn nounwind
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          55: define internal void @_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h317267e4903dea04E(%"[closure@std::rt::begin_panic<&str>::{closure#0}]"* noalias nocapture dereferenceable(12) %f) unnamed_addr #2 personality i32 (...)* @rust_eh_personality {
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
label:6'1              ?                                                                                                                                                                                                                                                        possible intended match
          56: start:
label:6'0     ~~~~~~
          57:  %0 = alloca { i8*, i32 }, align 4
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          58:  %_2 = alloca %"[closure@std::rt::begin_panic<&str>::{closure#0}]", align 4
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          59:  %1 = bitcast %"[closure@std::rt::begin_panic<&str>::{closure#0}]"* %_2 to i8*
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          60:  call void @llvm.lifetime.start.p0i8(i64 12, i8* %1)
label:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>

---
test result: FAILED. 194 passed; 1 failed; 75 ignored; 0 measured; 0 filtered out; finished in 1.39s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-wasm32-unknown-emscripten" "--suite" "codegen" "--mode" "codegen" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/latest/bin/node" "--npm" "/emsdk-portable/node/latest/bin/npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --host= --target wasm32-unknown-emscripten --exclude library/alloc
Build completed unsuccessfully in 0:25:46
