plain
test [assembly] assembly/asm/x86-types.rs#x86_64 ... ok

failures:

---- [assembly] assembly/static-relocation-model.rs#x64 stdout ----

error in revision `x64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s" "/checkout/src/test/assembly/static-relocation-model.rs" "--check-prefixes" "CHECK,x64"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/assembly/static-relocation-model.rs:37:9: error: x64: expected string not found in input
// x64: movb chaenomeles(%{{[a-z0-9]+}}), %{{[a-z]+}}
        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s:8:2: note: scanning from here
 .cfi_startproc
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s:9:2: note: possible intended match here
 movb chaenomeles, %al
 ^
/checkout/src/test/assembly/static-relocation-model.rs:48:9: error: x64: expected string not found in input
// x64: movb banana(%{{[a-z0-9]+}}), %{{[a-z]+}}
        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s:20:2: note: scanning from here
 .cfi_startproc
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s:21:2: note: possible intended match here
 movb banana, %al
 ^
/checkout/src/test/assembly/static-relocation-model.rs:59:9: error: x64: expected string not found in input
// x64: movq EXOCHORDA(%{{[a-z0-9]+}}), %{{[a-z]+}}
        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s:32:2: note: scanning from here
 .cfi_startproc
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s:33:2: note: possible intended match here
 movq EXOCHORDA, %rax
 ^
/checkout/src/test/assembly/static-relocation-model.rs:71:9: error: x64: expected string not found in input
// x64: movl $PIERIS, %{{[a-z]+}}
        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s:45:2: note: scanning from here
 .cfi_startproc
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s:46:2: note: possible intended match here
 movabsq $PIERIS, %rax


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/static-relocation-model.x64/static-relocation-model.s


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            1:  .text
            1:  .text
            2:  .file "static_relocation_model.3a1fbbbh-cgu.0"
            3:  .section .text.banana,"ax",@progbits
            4:  .globl banana
            5:  .p2align 4, 0x90
            6:  .type banana,@function
            7: banana:
            8:  .cfi_startproc
check:37'0      X~~~~~~~~~~~~~ error: no match found
            9:  movb chaenomeles, %al
check:37'0     ~~~~~~~~~~~~~~~~~~~~~~
check:37'1      ?                     possible intended match
           10:  retq
check:37'0     ~~~~~
           11: .Lfunc_end0:
check:37'0     ~~~~~~~~~~~~
           12:  .size banana, .Lfunc_end0-banana
check:37'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  .cfi_endproc
check:37'0     ~~~~~~~~~~~~~
           14: 
check:37'0     ~
           15:  .section .text.peach,"ax",@progbits
check:37'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  .globl peach
check:37'0     ~~~~~~~~~~~~~
           17:  .p2align 4, 0x90
check:37'0     ~~~~~~~~~~~~~~~~~
           18:  .type peach,@function
check:37'0     ~~~~~~~~~~~~~~~~~~~~~~
           19: peach:
check:37'0     ~~~~~~
           20:  .cfi_startproc
check:48'0      X~~~~~~~~~~~~~ error: no match found
           21:  movb banana, %al
check:48'0     ~~~~~~~~~~~~~~~~~
check:48'1      ?                possible intended match
           22:  retq
check:48'0     ~~~~~
           23: .Lfunc_end1:
check:48'0     ~~~~~~~~~~~~
           24:  .size peach, .Lfunc_end1-peach
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25:  .cfi_endproc
check:48'0     ~~~~~~~~~~~~~
           26: 
check:48'0     ~
           27:  .section .text.mango,"ax",@progbits
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  .globl mango
check:48'0     ~~~~~~~~~~~~~
           29:  .p2align 4, 0x90
check:48'0     ~~~~~~~~~~~~~~~~~
           30:  .type mango,@function
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~
           31: mango:
check:48'0     ~~~~~~
           32:  .cfi_startproc
check:59'0      X~~~~~~~~~~~~~ error: no match found
           33:  movq EXOCHORDA, %rax
check:59'0     ~~~~~~~~~~~~~~~~~~~~~
check:59'1      ?                    possible intended match
           34:  movb (%rax), %al
check:59'0     ~~~~~~~~~~~~~~~~~
           35:  retq
check:59'0     ~~~~~
           36: .Lfunc_end2:
check:59'0     ~~~~~~~~~~~~
           37:  .size mango, .Lfunc_end2-mango
check:59'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           38:  .cfi_endproc
check:59'0     ~~~~~~~~~~~~~
           39: 
check:59'0     ~
           40:  .section .text.orange,"ax",@progbits
check:59'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  .globl orange
check:59'0     ~~~~~~~~~~~~~~
           42:  .p2align 4, 0x90
check:59'0     ~~~~~~~~~~~~~~~~~
           43:  .type orange,@function
check:59'0     ~~~~~~~~~~~~~~~~~~~~~~~
           44: orange:
check:59'0     ~~~~~~~
           45:  .cfi_startproc
check:71'0      X~~~~~~~~~~~~~ error: no match found
           46:  movabsq $PIERIS, %rax
check:71'0     ~~~~~~~~~~~~~~~~~~~~~~
check:71'1      ?                     possible intended match
           47:  retq
check:71'0     ~~~~~
           48: .Lfunc_end3:
check:71'0     ~~~~~~~~~~~~
           49:  .size orange, .Lfunc_end3-orange
check:71'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50:  .cfi_endproc
check:71'0     ~~~~~~~~~~~~~
           51: 
check:71'0     ~
            .
            .
>>>>>>

---
test result: FAILED. 23 passed; 1 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.27s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:19:09
