plain
test [codegen] codegen/riscv-abi/riscv64-lp64d-abi.rs ... ignored
test [codegen] codegen/riscv-abi/riscv64-lp64f-lp64d-abi.rs ... ignored
test [codegen] codegen/pic-relocation-model.rs ... ok
test [codegen] codegen/personality_lifetimes.rs ... ok
test [codegen] codegen/pie-relocation-model.rs ... ok
test [codegen] codegen/packed.rs ... ok
test [codegen] codegen/nounwind.rs ... ok
test [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-1 ... ok
test [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-0 ... ok
---
test [assembly] assembly/x86_64-fortanix-unknown-sgx-lvi-inline-assembly.rs ... ignored
test [assembly] assembly/panic-no-unwind-no-uwtable.rs ... ok
test [assembly] assembly/static-relocation-model.rs#A64 ... ok
test [assembly] assembly/static-relocation-model.rs#ppc64le ... ok
test [assembly] assembly/pie-relocation-model.rs#x64 ... FAILED
test [assembly] assembly/asm/aarch64-types.rs ... ok
test [assembly] assembly/static-relocation-model.rs#x64 ... ok
test [assembly] assembly/pic-relocation-model.rs#x64 ... FAILED
test [assembly] assembly/target-feature-multiple.rs#TWOFLAGS ... ok
---
test [assembly] assembly/asm/x86-types.rs#i686 ... ok

failures:

---- [assembly] assembly/pie-relocation-model.rs#x64 stdout ----

error in revision `x64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/pie-relocation-model.s" "/checkout/src/test/assembly/pie-relocation-model.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,x64"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/assembly/pie-relocation-model.rs:12:11: error: CHECK: expected string not found in input
// CHECK: jmp other_fn
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/pie-relocation-model.s:7:15: note: scanning from here
call_other_fn:
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/pie-relocation-model.s:11:4: note: possible intended match here
 callq other_fn


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/pie-relocation-model.s
Check file: /checkout/src/test/assembly/pie-relocation-model.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            1:  .text 
            1:  .text 
            2:  .file "pie_relocation_model.8d7ed54a-cgu.0" 
            3:  .section .text.call_other_fn,"ax",@progbits 
            4:  .globl call_other_fn 
            5:  .p2align 4, 0x90 
            6:  .type call_other_fn,@function 
            7: call_other_fn: 
check:12'0                   X error: no match found
            8:  .cfi_startproc 
check:12'0     ~~~~~~~~~~~~~~~~
            9:  pushq %rax 
check:12'0     ~~~~~~~~~~~~
           10:  .cfi_def_cfa_offset 16 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           11:  callq other_fn 
check:12'0     ~~~~~~~~~~~~~~~~
check:12'1        ?             possible intended match
           12:  movb %al, 7(%rsp) 
check:12'0     ~~~~~~~~~~~~~~~~~~~
           13:  movb 7(%rsp), %al 
check:12'0     ~~~~~~~~~~~~~~~~~~~
           14:  popq %rcx 
check:12'0     ~~~~~~~~~~~
           15:  .cfi_def_cfa_offset 8 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~
           16:  retq 
check:12'0     ~~~~~~
            .
            .
>>>>>>


------------------------------------------


---- [assembly] assembly/pic-relocation-model.rs#x64 stdout ----

error in revision `x64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s" "/checkout/src/test/assembly/pic-relocation-model.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,x64"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/assembly/pic-relocation-model.rs:11:11: error: CHECK: expected string not found in input
// CHECK: jmpq *other_fn@GOTPCREL(%rip)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s:7:15: note: scanning from here
call_other_fn:
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s:11:3: note: possible intended match here
 callq *other_fn@GOTPCREL(%rip)


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s
Check file: /checkout/src/test/assembly/pic-relocation-model.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            1:  .text 
            1:  .text 
            2:  .file "pic_relocation_model.28b99809-cgu.0" 
            3:  .section .text.call_other_fn,"ax",@progbits 
            4:  .globl call_other_fn 
            5:  .p2align 4, 0x90 
            6:  .type call_other_fn,@function 
            7: call_other_fn: 
check:11'0                   X error: no match found
            8:  .cfi_startproc 
check:11'0     ~~~~~~~~~~~~~~~~
            9:  pushq %rax 
check:11'0     ~~~~~~~~~~~~
           10:  .cfi_def_cfa_offset 16 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           11:  callq *other_fn@GOTPCREL(%rip) 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:11'1       ?                              possible intended match
           12:  movb %al, 7(%rsp) 
check:11'0     ~~~~~~~~~~~~~~~~~~~
           13:  movb 7(%rsp), %al 
check:11'0     ~~~~~~~~~~~~~~~~~~~
           14:  popq %rcx 
check:11'0     ~~~~~~~~~~~
           15:  .cfi_def_cfa_offset 8 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~
           16:  retq 
check:11'0     ~~~~~~
            .
            .
>>>>>>


------------------------------------------



failures:
    [assembly] assembly/pic-relocation-model.rs#x64
    [assembly] assembly/pie-relocation-model.rs#x64
test result: FAILED. 28 passed; 2 failed; 13 ignored; 0 measured; 0 filtered out; finished in 0.28s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:18:02
