plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 136 tests
iiiiii..iiiiiiiiiiF.F....FF......................................................................... 100/136
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [assembly] assembly/asm/s390x-types.rs#s390x stdout ----

error in revision `s390x`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/s390x-types.s390x/s390x-types.s" "/checkout/src/test/assembly/asm/s390x-types.rs" "--check-prefixes" "CHECK,NONMSVC,s390x"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/assembly/asm/s390x-types.rs:91:17: error: CHECK-LABEL: expected string not found in input
// CHECK-LABEL: reg_i8:
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/s390x-types.s390x/s390x-types.s:371:2: note: scanning from here
 .cfi_startproc
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/s390x-types.s390x/s390x-types.s:390:10: note: possible intended match here
 .ascii "reg_i8"

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/s390x-types.s390x/s390x-types.s
Check file: /checkout/src/test/assembly/asm/s390x-types.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          366:  .section .text.sym_static,"ax",@progbits
          367:  .globl sym_static
          368:  .p2align 4
          369:  .type sym_static,@function
          370: sym_static:
          371:  .cfi_startproc
label:91'0      X~~~~~~~~~~~~~ error: no match found
          372:  stmg %r14, %r15, 112(%r15)
label:91'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          373:  .cfi_offset %r14, -48
label:91'0     ~~~~~~~~~~~~~~~~~~~~~~
          374:  .cfi_offset %r15, -40
label:91'0     ~~~~~~~~~~~~~~~~~~~~~~
          375:  aghi %r15, -160
label:91'0     ~~~~~~~~~~~~~~~~
          376:  .cfi_def_cfa_offset 320
label:91'0     ~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
          385: 
          385: 
label:91'0     ~
          386:  .type .L__unnamed_1,@object
label:91'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          387:  .section .rodata..L__unnamed_1,"a",@progbits
label:91'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          388:  .p2align 1
label:91'0     ~~~~~~~~~~~
          389: .L__unnamed_1:
label:91'0     ~~~~~~~~~~~~~~
          390:  .ascii "reg_i8"
label:91'0     ~~~~~~~~~~~~~~~~
label:91'1              ?       possible intended match
          391:  .size .L__unnamed_1, 6
label:91'0     ~~~~~~~~~~~~~~~~~~~~~~~
          392: 
label:91'0     ~
          393:  .type .L__unnamed_2,@object
label:91'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          394:  .section .rodata..L__unnamed_2,"a",@progbits
label:91'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          395:  .p2align 1
label:91'0     ~~~~~~~~~~~
            .
            .
>>>>>>


------------------------------------------


---- [assembly] assembly/asm/mips-types.rs#mips32 stdout ----

error in revision `mips32`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/mips-types.mips32/mips-types.s" "/checkout/src/test/assembly/asm/mips-types.rs" "--check-prefixes" "CHECK,NONMSVC,mips32"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/assembly/asm/mips-types.rs:113:17: error: CHECK-LABEL: expected string not found in input
// CHECK-LABEL: reg_f32:
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/mips-types.mips32/mips-types.s:1054:2: note: scanning from here
 .cfi_startproc
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/mips-types.mips32/mips-types.s:1086:10: note: possible intended match here
 .ascii "reg_f32"

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/mips-types.mips32/mips-types.s
Check file: /checkout/src/test/assembly/asm/mips-types.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
          1049:  .type sym_fn_32,@function
          1050:  .set nomicromips
          1051:  .set nomips16
          1052:  .ent sym_fn_32
          1053: sym_fn_32:
          1054:  .cfi_startproc
label:113'0      X~~~~~~~~~~~~~ error: no match found
          1055:  .frame $sp,0,$ra
label:113'0     ~~~~~~~~~~~~~~~~~
          1056:  .mask 0x00000000,0
label:113'0     ~~~~~~~~~~~~~~~~~~~
          1057:  .fmask 0x00000000,0
label:113'0     ~~~~~~~~~~~~~~~~~~~~
          1058:  .set noreorder
label:113'0     ~~~~~~~~~~~~~~~
          1059:  .set nomacro
label:113'0     ~~~~~~~~~~~~~
             .
             .
             .
          1081:  .cfi_endproc
label:113'0     ~~~~~~~~~~~~~
          1082: 
label:113'0     ~
          1083:  .type $__unnamed_1,@object
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          1084:  .section ".rodata.$__unnamed_1","a",@progbits
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          1085: $__unnamed_1:
label:113'0     ~~~~~~~~~~~~~
          1086:  .ascii "reg_f32"
label:113'0     ~~~~~~~~~~~~~~~~~
label:113'1              ?        possible intended match
          1087:  .size $__unnamed_1, 7
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~
          1088: 
label:113'0     ~
          1089:  .type $__unnamed_2,@object
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          1090:  .section ".rodata.$__unnamed_2","a",@progbits
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          1091: $__unnamed_2:
label:113'0     ~~~~~~~~~~~~~
             .
             .
>>>>>>


------------------------------------------


---- [assembly] assembly/asm/hexagon-types.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/hexagon-types/hexagon-types.s" "/checkout/src/test/assembly/asm/hexagon-types.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/assembly/asm/hexagon-types.rs:123:17: error: CHECK-LABEL: expected string not found in input
// CHECK-LABEL: reg_ptr:
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/hexagon-types/hexagon-types.s:397:2: note: scanning from here
 .cfi_startproc
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/hexagon-types/hexagon-types.s:424:10: note: possible intended match here
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/hexagon-types/hexagon-types.s:424:10: note: possible intended match here
 .ascii "reg_ptr"

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/hexagon-types/hexagon-types.s
Check file: /checkout/src/test/assembly/asm/hexagon-types.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
           392:  .section .text.packet,"ax",@progbits
           393:  .globl packet
           394:  .p2align 4
           395:  .type packet,@function
           396: packet:
           397:  .cfi_startproc
label:123'0      X~~~~~~~~~~~~~ error: no match found
           398:  {
label:123'0     ~~
           399:  allocframe(r29,#8):raw
label:123'0     ~~~~~~~~~~~~~~~~~~~~~~~
           400:  }
label:123'0     ~~
           401:  .cfi_def_cfa r30, 8
label:123'0     ~~~~~~~~~~~~~~~~~~~~
           402:  .cfi_offset r31, -4
label:123'0     ~~~~~~~~~~~~~~~~~~~~
             .
             .
             .
           419:  .cfi_endproc
label:123'0     ~~~~~~~~~~~~~
           420: 
label:123'0     ~
           421:  .type .L__unnamed_1,@object
label:123'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           422:  .section .rodata..L__unnamed_1,"a",@progbits
label:123'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           423: .L__unnamed_1:
label:123'0     ~~~~~~~~~~~~~~
           424:  .ascii "reg_ptr"
label:123'0     ~~~~~~~~~~~~~~~~~
label:123'1              ?        possible intended match
           425:  .size .L__unnamed_1, 7
label:123'0     ~~~~~~~~~~~~~~~~~~~~~~~
           426: 
label:123'0     ~
           427:  .type .L__unnamed_2,@object
label:123'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           428:  .section .rodata..L__unnamed_2,"a",@progbits
label:123'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           429: .L__unnamed_2:
label:123'0     ~~~~~~~~~~~~~~
             .
             .
>>>>>>


------------------------------------------


---- [assembly] assembly/asm/mips-types.rs#mips64 stdout ----

error in revision `mips64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/mips-types.mips64/mips-types.s" "/checkout/src/test/assembly/asm/mips-types.rs" "--check-prefixes" "CHECK,NONMSVC,mips64"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/assembly/asm/mips-types.rs:113:17: error: CHECK-LABEL: expected string not found in input
// CHECK-LABEL: reg_f32:
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/mips-types.mips64/mips-types.s:1230:2: note: scanning from here
 .cfi_startproc
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/mips-types.mips64/mips-types.s:1262:10: note: possible intended match here
 .ascii "reg_f32"

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/mips-types.mips64/mips-types.s
Check file: /checkout/src/test/assembly/asm/mips-types.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
          1225:  .type sym_fn_64,@function
          1226:  .set nomicromips
          1227:  .set nomips16
          1228:  .ent sym_fn_64
          1229: sym_fn_64:
          1230:  .cfi_startproc
label:113'0      X~~~~~~~~~~~~~ error: no match found
          1231:  .frame $sp,0,$ra
label:113'0     ~~~~~~~~~~~~~~~~~
          1232:  .mask 0x00000000,0
label:113'0     ~~~~~~~~~~~~~~~~~~~
          1233:  .fmask 0x00000000,0
label:113'0     ~~~~~~~~~~~~~~~~~~~~
          1234:  .set noreorder
label:113'0     ~~~~~~~~~~~~~~~
          1235:  .set nomacro
label:113'0     ~~~~~~~~~~~~~
             .
             .
             .
          1257:  .cfi_endproc
label:113'0     ~~~~~~~~~~~~~
          1258: 
label:113'0     ~
          1259:  .type .L__unnamed_1,@object
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          1260:  .section .rodata..L__unnamed_1,"a",@progbits
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          1261: .L__unnamed_1:
label:113'0     ~~~~~~~~~~~~~~
          1262:  .ascii "reg_f32"
label:113'0     ~~~~~~~~~~~~~~~~~
label:113'1              ?        possible intended match
          1263:  .size .L__unnamed_1, 7
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~~
          1264: 
label:113'0     ~
          1265:  .type .L__unnamed_2,@object
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          1266:  .section .rodata..L__unnamed_2,"a",@progbits
label:113'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          1267: .L__unnamed_2:
label:113'0     ~~~~~~~~~~~~~~
             .
             .
>>>>>>

---
test result: FAILED. 113 passed; 4 failed; 19 ignored; 0 measured; 0 filtered out; finished in 0.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:13
