plain
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...................................iii...............
failures:

---- [assembly] src/test/assembly/align_offset.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s" "/checkout/src/test/assembly/align_offset.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/align_offset.rs:7:11: error: CHECK: expected string not found in input
// CHECK: leaq 31
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:401:38: note: scanning from here
 .section .text.align_offset_byte_ptr,"ax",@progbits
                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:402:4: note: possible intended match here
 .globl align_offset_byte_ptr
   ^
/checkout/src/test/assembly/align_offset.rs:16:11: error: CHECK: expected string not found in input
// CHECK: leaq 31
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:418:40: note: scanning from here
 .section .text.align_offset_byte_slice,"ax",@progbits
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:419:4: note: possible intended match here
 .globl align_offset_byte_slice
   ^
/checkout/src/test/assembly/align_offset.rs:25:11: error: CHECK: expected string not found in input
// CHECK: leaq 31
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:437:38: note: scanning from here
 .section .text.align_offset_word_ptr,"ax",@progbits
                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:438:4: note: possible intended match here
 .globl align_offset_word_ptr
   ^
/checkout/src/test/assembly/align_offset.rs:38:11: error: CHECK: expected string not found in input
// CHECK: leaq 31
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:454:40: note: scanning from here
 .section .text.align_offset_word_slice,"ax",@progbits
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:475:6: note: possible intended match here
     ^


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s
Check file: /checkout/src/test/assembly/align_offset.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          396:  retq
          397: .Lfunc_end16:
          398:  .size _ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17h5a4735dbb363153bE, .Lfunc_end16-_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17h5a4735dbb363153bE
          399:  .cfi_endproc
          400: 
          401:  .section .text.align_offset_byte_ptr,"ax",@progbits
check:7'0                                           X~~~~~~~~~~~~~~ error: no match found
          402:  .globl align_offset_byte_ptr
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:7'1         ?                          possible intended match
          403:  .p2align 4, 0x90
check:7'0      ~~~~~~~~~~~~~~~~~
          404:  .type align_offset_byte_ptr,@function
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          405: align_offset_byte_ptr:
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~
          406:  .cfi_startproc
check:7'0      ~~~~~~~~~~~~~~~
          407:  pushq %rax
check:7'0      ~~~~~~~~~~~
            .
            .
          413:  retq
          413:  retq
check:7'0      ~~~~~
          414: .Lfunc_end17:
check:7'0      ~~~~~~~~~~~~~
          415:  .size align_offset_byte_ptr, .Lfunc_end17-align_offset_byte_ptr
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          416:  .cfi_endproc
check:7'0      ~~~~~~~~~~~~~
          417: 
check:7'0      ~
          418:  .section .text.align_offset_byte_slice,"ax",@progbits
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'0                                            X~~~~~~~~~~~~~~ error: no match found
          419:  .globl align_offset_byte_slice
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'1        ?                            possible intended match
          420:  .p2align 4, 0x90
check:16'0     ~~~~~~~~~~~~~~~~~
          421:  .type align_offset_byte_slice,@function
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          422: align_offset_byte_slice:
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          423:  .cfi_startproc
check:16'0     ~~~~~~~~~~~~~~~
          424:  pushq %rax
check:16'0     ~~~~~~~~~~~
            .
            .
          432:  retq
          432:  retq
check:16'0     ~~~~~
          433: .Lfunc_end18:
check:16'0     ~~~~~~~~~~~~~
          434:  .size align_offset_byte_slice, .Lfunc_end18-align_offset_byte_slice
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          435:  .cfi_endproc
check:16'0     ~~~~~~~~~~~~~
          436: 
check:16'0     ~
          437:  .section .text.align_offset_word_ptr,"ax",@progbits
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:25'0                                          X~~~~~~~~~~~~~~ error: no match found
          438:  .globl align_offset_word_ptr
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:25'1        ?                          possible intended match
          439:  .p2align 4, 0x90
check:25'0     ~~~~~~~~~~~~~~~~~
          440:  .type align_offset_word_ptr,@function
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          441: align_offset_word_ptr:
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~
          442:  .cfi_startproc
check:25'0     ~~~~~~~~~~~~~~~
          443:  pushq %rax
check:25'0     ~~~~~~~~~~~
            .
            .
          449:  retq
          449:  retq
check:25'0     ~~~~~
          450: .Lfunc_end19:
check:25'0     ~~~~~~~~~~~~~
          451:  .size align_offset_word_ptr, .Lfunc_end19-align_offset_word_ptr
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          452:  .cfi_endproc
check:25'0     ~~~~~~~~~~~~~
          453: 
check:25'0     ~
          454:  .section .text.align_offset_word_slice,"ax",@progbits
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'0                                            X~~~~~~~~~~~~~~ error: no match found
          455:  .globl align_offset_word_slice
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          456:  .p2align 4, 0x90
check:38'0     ~~~~~~~~~~~~~~~~~
          457:  .type align_offset_word_slice,@function
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          458: align_offset_word_slice:
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          459:  .cfi_startproc
check:38'0     ~~~~~~~~~~~~~~~
            .
            .
            .
          470:  .size align_offset_word_slice, .Lfunc_end20-align_offset_word_slice
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          471:  .cfi_endproc
check:38'0     ~~~~~~~~~~~~~
          472: 
check:38'0     ~
          473:  .type .L__unnamed_2,@object
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          474:  .section .rodata..L__unnamed_2,"a",@progbits
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          475:  .p2align 3
check:38'0     ~~~~~~~~~~~
check:38'1          ?      possible intended match
          476: .L__unnamed_2:
check:38'0     ~~~~~~~~~~~~~~
          477:  .size .L__unnamed_2, 0
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~
          478: 
check:38'0     ~
          479:  .type .L__unnamed_3,@object
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          480:  .section .rodata.cst8,"aM",@progbits,8
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
