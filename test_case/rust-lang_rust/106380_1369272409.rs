plain
 finished in 0.627 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 152 tests
iii............iiiiiiiiiiiii.........F..F............................................... 88/152
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [assembly] src/test/assembly/pie-relocation-model.rs#x64 stdout ----

error in revision `x64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/pie-relocation-model.s" "/checkout/src/test/assembly/pie-relocation-model.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,x64" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/pie-relocation-model.rs:29:11: error: CHECK: expected string not found in input
// CHECK: callq *foreign_fn@GOTPCREL(%rip)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/pie-relocation-model.s:18:10: note: scanning from here
other_fn:
         ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/pie-relocation-model.s:22:2: note: possible intended match here
 callq foreign_fn@PLT


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/pie-relocation-model.s


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            1:  .text 
            1:  .text 
            2:  .file "pie_relocation_model.c79b56be-cgu.0" 
            3:  .section .text.call_other_fn,"ax",@progbits 
            4:  .globl call_other_fn 
            5:  .p2align 4, 0x90 
            6:  .type call_other_fn,@function 
            7: call_other_fn: 
            8:  .cfi_startproc 
            9:  jmp other_fn 
           10: .Lfunc_end0: 
           11:  .size call_other_fn, .Lfunc_end0-call_other_fn 
           12:  .cfi_endproc 
           13:  
           14:  .section .text.other_fn,"ax",@progbits 
           15:  .globl other_fn 
           16:  .p2align 4, 0x90 
           17:  .type other_fn,@function 
           18: other_fn: 
check:29'0              X error: no match found
           19:  .cfi_startproc 
check:29'0     ~~~~~~~~~~~~~~~~
           20:  pushq %rax 
check:29'0     ~~~~~~~~~~~~
           21:  .cfi_def_cfa_offset 16 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           22:  callq foreign_fn@PLT 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~
check:29'1      ?                     possible intended match
           23:  popq %rcx 
check:29'0     ~~~~~~~~~~~
           24:  .cfi_def_cfa_offset 8 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~
           25:  retq 
check:29'0     ~~~~~~
           26: .Lfunc_end1: 
check:29'0     ~~~~~~~~~~~~~
           27:  .size other_fn, .Lfunc_end1-other_fn 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  .cfi_endproc 
check:29'0     ~~~~~~~~~~~~~~
           29:  
check:29'0     ~
           30:  .section ".note.GNU-stack","",@progbits 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



---- [assembly] src/test/assembly/pic-relocation-model.rs#x64 stdout ----

error in revision `x64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s" "/checkout/src/test/assembly/pic-relocation-model.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,x64" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/pic-relocation-model.rs:17:11: error: CHECK: expected string not found in input
// CHECK: {{(jmpq|callq)}} *other_fn@GOTPCREL(%rip)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s:7:15: note: scanning from here
call_other_fn:
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s:9:2: note: possible intended match here
 jmp other_fn@PLT
/checkout/src/test/assembly/pic-relocation-model.rs:26:11: error: CHECK: expected string not found in input
/checkout/src/test/assembly/pic-relocation-model.rs:26:11: error: CHECK: expected string not found in input
// CHECK: callq *foreign_fn@GOTPCREL(%rip)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s:18:10: note: scanning from here
other_fn:
         ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s:22:2: note: possible intended match here
 callq foreign_fn@PLT


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            1:  .text 
            1:  .text 
            2:  .file "pic_relocation_model.c32f04f8-cgu.0" 
            3:  .section .text.call_other_fn,"ax",@progbits 
            4:  .globl call_other_fn 
            5:  .p2align 4, 0x90 
            6:  .type call_other_fn,@function 
            7: call_other_fn: 
check:17'0                   X error: no match found
            8:  .cfi_startproc 
check:17'0     ~~~~~~~~~~~~~~~~
            9:  jmp other_fn@PLT 
check:17'0     ~~~~~~~~~~~~~~~~~~
check:17'1      ?                 possible intended match
           10: .Lfunc_end0: 
check:17'0     ~~~~~~~~~~~~~
           11:  .size call_other_fn, .Lfunc_end0-call_other_fn 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12:  .cfi_endproc 
check:17'0     ~~~~~~~~~~~~~~
           13:  
check:17'0     ~
           14:  .section .text.other_fn,"ax",@progbits 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15:  .globl other_fn 
check:17'0     ~~~~~~~~~~~~~~~~~
           16:  .p2align 4, 0x90 
check:17'0     ~~~~~~~~~~~~~~~~~~
           17:  .type other_fn,@function 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: other_fn: 
check:17'0     ~~~~~~~~~
check:26'0              X error: no match found
           19:  .cfi_startproc 
check:26'0     ~~~~~~~~~~~~~~~~
           20:  pushq %rax 
check:26'0     ~~~~~~~~~~~~
           21:  .cfi_def_cfa_offset 16 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           22:  callq foreign_fn@PLT 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~
check:26'1      ?                     possible intended match
           23:  popq %rcx 
check:26'0     ~~~~~~~~~~~
           24:  .cfi_def_cfa_offset 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~
           25:  retq 
check:26'0     ~~~~~~
           26: .Lfunc_end1: 
check:26'0     ~~~~~~~~~~~~~
           27:  .size other_fn, .Lfunc_end1-other_fn 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  .cfi_endproc 
check:26'0     ~~~~~~~~~~~~~~
           29:  
check:26'0     ~
           30:  .section ".note.GNU-stack","",@progbits 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



