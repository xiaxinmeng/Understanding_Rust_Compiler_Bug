plain
test [codegen] codegen/lto-removes-invokes.rs ... ok

failures:

---- [codegen] codegen/dst-vtable-align-nonzero.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/dst-vtable-align-nonzero/dst-vtable-align-nonzero.ll" "/checkout/src/test/codegen/dst-vtable-align-nonzero.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/dst-vtable-align-nonzero.rs:25:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: icmp
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/dst-vtable-align-nonzero/dst-vtable-align-nonzero.ll:21:7: note: found here
 %7 = icmp ugt i64 1, %5
      ^~~~
/checkout/src/test/codegen/dst-vtable-align-nonzero.rs:26:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: select
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/dst-vtable-align-nonzero/dst-vtable-align-nonzero.ll:22:7: note: found here
 %8 = select i1 %7, i64 1, i64 %5


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/dst-vtable-align-nonzero/dst-vtable-align-nonzero.ll
Check file: /checkout/src/test/codegen/dst-vtable-align-nonzero.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       16:  %2 = load i64, i64* %1, align 8, !invariant.load !2 
       17:  %3 = bitcast [3 x i64]* %x.1 to i64* 
       18:  %4 = getelementptr inbounds i64, i64* %3, i64 2 
       19:  %5 = load i64, i64* %4, align 8, !range !3, !invariant.load !2 
       20:  %6 = add i64 1, %2 
       21:  %7 = icmp ugt i64 1, %5 
not:25           !~~~                error: no match expected
       22:  %8 = select i1 %7, i64 1, i64 %5 
not:26           !~~~~~                       error: no match expected
       23:  %9 = sub i64 %8, 1 
       24:  %10 = add i64 %6, %9 
       25:  %11 = sub i64 0, %8 
       26:  %12 = and i64 %10, %11 
       27:  %13 = sub i64 %8, 1 
        .
        .
>>>>>>

---
test result: FAILED. 266 passed; 1 failed; 40 ignored; 0 measured; 0 filtered out; finished in 4.11s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.59.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:19:34
