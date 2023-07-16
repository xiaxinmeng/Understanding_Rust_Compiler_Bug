plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu

failures:

---- [codegen] codegen/mem-replace-direct-memcpy.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll" "/checkout/src/test/codegen/mem-replace-direct-memcpy.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/mem-replace-direct-memcpy.rs:20:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %{{.*}}, i8* align 1 %src, i64 1, i1 false)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:123:1: note: scanning from here
; Function Attrs: inlinehint nonlazybind uwtable
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:139:2: note: possible intended match here
 call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %_4, i8* align 1 %src, i32 1, i1 false)


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll
Check file: /checkout/src/test/codegen/mem-replace-direct-memcpy.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          118: bb5: ; preds = %bb6
          119:  br label %bb4
          120: }
          122: ; core::ptr::read
          122: ; core::ptr::read
          123: ; Function Attrs: inlinehint nonlazybind uwtable
check:20'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          124: define internal i8 @_ZN4core3ptr4read17h07008b88eda09086E(i8* %src) unnamed_addr #1 {
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          125: start:
check:20'0     ~~~~~~
          126:  %tmp = alloca i8, align 1
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          127:  call void @llvm.lifetime.start.p0i8(i64 1, i8* %tmp)
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          128: ; call core::mem::maybe_uninit::MaybeUninit<T>::uninit
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
          134: ; call core::mem::maybe_uninit::MaybeUninit<T>::as_mut_ptr
          134: ; call core::mem::maybe_uninit::MaybeUninit<T>::as_mut_ptr
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          135:  %_4 = call i8* @"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h2117c9ea0386edf5E"(i8* noalias align 1 dereferenceable(1) %tmp)
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          136:  br label %bb2
check:20'0     ~~~~~~~~~~~~~~
          137: 
check:20'0     ~
          138: bb2: ; preds = %bb1
check:20'0     ~~~~~~~~~~~~~~~~~~~
          139:  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %_4, i8* align 1 %src, i32 1, i1 false)
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:20'1      ?                                                                                        possible intended match
          140:  %_6 = load i8, i8* %tmp, align 1
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          141: ; call core::mem::maybe_uninit::MaybeUninit<T>::assume_init
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          142:  %1 = call i8 @"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17h417caeeba940ae76E"(i8 %_6, %"std::panic::Location"* noalias readonly align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc20 to %"std::panic::Location"*))
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          143:  br label %bb3
check:20'0     ~~~~~~~~~~~~~~
          144: 
check:20'0     ~
            .
            .
>>>>>>

---
test result: FAILED. 228 passed; 1 failed; 55 ignored; 0 measured; 0 filtered out; finished in 3.00s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-i586-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.56.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:19:49
