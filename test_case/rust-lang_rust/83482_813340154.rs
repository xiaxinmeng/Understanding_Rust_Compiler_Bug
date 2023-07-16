plain
test [assembly] assembly/asm/x86-types.rs#i686 ... ok

failures:

---- [assembly] assembly/panic-no-unwind-no-uwtable.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/panic-no-unwind-no-uwtable/panic-no-unwind-no-uwtable.s" "/checkout/src/test/assembly/panic-no-unwind-no-uwtable.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/assembly/panic-no-unwind-no-uwtable.rs:7:15: error: CHECK-NOT: excluded string found in input
// CHECK-NOT: .cfi_startproc
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/panic-no-unwind-no-uwtable/panic-no-unwind-no-uwtable.s:8:2: note: found here
 .cfi_startproc


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/panic-no-unwind-no-uwtable/panic-no-unwind-no-uwtable.s
Check file: /checkout/src/test/assembly/panic-no-unwind-no-uwtable.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
       1:  .text
       1:  .text
       2:  .file "panic_no_unwind_no_uwtable.3a1fbbbh-cgu.0"
       3:  .section .text._ZN26panic_no_unwind_no_uwtable3foo17hd58956e483392815E,"ax",@progbits
       4:  .globl _ZN26panic_no_unwind_no_uwtable3foo17hd58956e483392815E
       5:  .p2align 4, 0x90
       6:  .type _ZN26panic_no_unwind_no_uwtable3foo17hd58956e483392815E,@function
       7: _ZN26panic_no_unwind_no_uwtable3foo17hd58956e483392815E:
       8:  .cfi_startproc
not:7      !~~~~~~~~~~~~~ error: no match expected
       9:  retq
      10: .Lfunc_end0:
      11:  .size _ZN26panic_no_unwind_no_uwtable3foo17hd58956e483392815E, .Lfunc_end0-_ZN26panic_no_unwind_no_uwtable3foo17hd58956e483392815E
      12:  .cfi_endproc
      13: 
      14:  .section ".note.GNU-stack","",@progbits
>>>>>>
------------------------------------------




failures:
    [assembly] assembly/panic-no-unwind-no-uwtable.rs
test result: FAILED. 20 passed; 1 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.21s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:19:32
