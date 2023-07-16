plain
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu

failures:

---- [assembly] src/test/assembly/align_offset.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s" "/checkout/src/test/assembly/align_offset.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/align_offset.rs:7:11: error: CHECK: expected string not found in input
// CHECK: leaq 31
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:459:38: note: scanning from here
 .section .text.align_offset_byte_ptr,"ax",@progbits
                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:466:2: note: possible intended match here
 leal 31(%ecx), %eax
 ^
/checkout/src/test/assembly/align_offset.rs:16:11: error: CHECK: expected string not found in input
// CHECK: leaq 31
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:474:40: note: scanning from here
 .section .text.align_offset_byte_slice,"ax",@progbits
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:481:2: note: possible intended match here
 leal 31(%ecx), %eax
 ^
/checkout/src/test/assembly/align_offset.rs:25:11: error: CHECK: expected string not found in input
// CHECK: leaq 31
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:489:38: note: scanning from here
 .section .text.align_offset_word_ptr,"ax",@progbits
                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:496:2: note: possible intended match here
 leal 31(%eax), %ecx
 ^
/checkout/src/test/assembly/align_offset.rs:38:11: error: CHECK: expected string not found in input
// CHECK: leaq 31
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:509:40: note: scanning from here
 .section .text.align_offset_word_slice,"ax",@progbits
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:516:2: note: possible intended match here
 leal 31(%ecx), %eax


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s
Check file: /checkout/src/test/assembly/align_offset.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          454:  retl 
          455: .Lfunc_end5: 
          456:  .size _ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$12align_offset7rt_impl17hed181ec11423b42cE, .Lfunc_end5-_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$12align_offset7rt_impl17hed181ec11423b42cE 
          457:  .cfi_endproc 
          458:  
          459:  .section .text.align_offset_byte_ptr,"ax",@progbits 
check:7'0                                           X~~~~~~~~~~~~~~~ error: no match found
          460:  .globl align_offset_byte_ptr 
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          461:  .p2align 4, 0x90 
check:7'0      ~~~~~~~~~~~~~~~~~~
          462:  .type align_offset_byte_ptr,@function 
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          463: align_offset_byte_ptr: 
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~
          464:  .cfi_startproc 
check:7'0      ~~~~~~~~~~~~~~~~
          465:  movl 4(%esp), %ecx 
check:7'0      ~~~~~~~~~~~~~~~~~~~~
          466:  leal 31(%ecx), %eax 
check:7'0      ~~~~~~~~~~~~~~~~~~~~~
check:7'1       ?                    possible intended match
          467:  andl $-32, %eax 
check:7'0      ~~~~~~~~~~~~~~~~~
          468:  subl %ecx, %eax 
check:7'0      ~~~~~~~~~~~~~~~~~
          469:  retl 
check:7'0      ~~~~~~
          470: .Lfunc_end6: 
check:7'0      ~~~~~~~~~~~~~
          471:  .size align_offset_byte_ptr, .Lfunc_end6-align_offset_byte_ptr 
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          472:  .cfi_endproc 
check:7'0      ~~~~~~~~~~~~~~
          473:  
check:7'0      ~
          474:  .section .text.align_offset_byte_slice,"ax",@progbits 
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'0                                            X~~~~~~~~~~~~~~~ error: no match found
          475:  .globl align_offset_byte_slice 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          476:  .p2align 4, 0x90 
check:16'0     ~~~~~~~~~~~~~~~~~~
          477:  .type align_offset_byte_slice,@function 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          478: align_offset_byte_slice: 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          479:  .cfi_startproc 
check:16'0     ~~~~~~~~~~~~~~~~
          480:  movl 4(%esp), %ecx 
check:16'0     ~~~~~~~~~~~~~~~~~~~~
          481:  leal 31(%ecx), %eax 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~
check:16'1      ?                    possible intended match
          482:  andl $-32, %eax 
check:16'0     ~~~~~~~~~~~~~~~~~
          483:  subl %ecx, %eax 
check:16'0     ~~~~~~~~~~~~~~~~~
          484:  retl 
check:16'0     ~~~~~~
          485: .Lfunc_end7: 
check:16'0     ~~~~~~~~~~~~~
          486:  .size align_offset_byte_slice, .Lfunc_end7-align_offset_byte_slice 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          487:  .cfi_endproc 
check:16'0     ~~~~~~~~~~~~~~
          488:  
check:16'0     ~
          489:  .section .text.align_offset_word_ptr,"ax",@progbits 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:25'0                                          X~~~~~~~~~~~~~~~ error: no match found
          490:  .globl align_offset_word_ptr 
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          491:  .p2align 4, 0x90 
check:25'0     ~~~~~~~~~~~~~~~~~~
          492:  .type align_offset_word_ptr,@function 
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          493: align_offset_word_ptr: 
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~
          494:  .cfi_startproc 
check:25'0     ~~~~~~~~~~~~~~~~
          495:  movl 4(%esp), %eax 
check:25'0     ~~~~~~~~~~~~~~~~~~~~
          496:  leal 31(%eax), %ecx 
check:25'0     ~~~~~~~~~~~~~~~~~~~~~
check:25'1      ?                    possible intended match
          497:  andl $-32, %ecx 
check:25'0     ~~~~~~~~~~~~~~~~~
          498:  subl %eax, %ecx 
check:25'0     ~~~~~~~~~~~~~~~~~
          499:  andl $3, %eax 
check:25'0     ~~~~~~~~~~~~~~~
          500:  shrl $2, %ecx 
check:25'0     ~~~~~~~~~~~~~~~
          501:  negl %eax 
check:25'0     ~~~~~~~~~~~
            .
            .
          504:  retl 
          504:  retl 
check:25'0     ~~~~~~
          505: .Lfunc_end8: 
check:25'0     ~~~~~~~~~~~~~
          506:  .size align_offset_word_ptr, .Lfunc_end8-align_offset_word_ptr 
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          507:  .cfi_endproc 
check:25'0     ~~~~~~~~~~~~~~
          508:  
check:25'0     ~
          509:  .section .text.align_offset_word_slice,"ax",@progbits 
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'0                                            X~~~~~~~~~~~~~~~ error: no match found
          510:  .globl align_offset_word_slice 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          511:  .p2align 4, 0x90 
check:38'0     ~~~~~~~~~~~~~~~~~~
          512:  .type align_offset_word_slice,@function 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          513: align_offset_word_slice: 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          514:  .cfi_startproc 
check:38'0     ~~~~~~~~~~~~~~~~
          515:  movl 4(%esp), %ecx 
check:38'0     ~~~~~~~~~~~~~~~~~~~~
          516:  leal 31(%ecx), %eax 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~
check:38'1      ?                    possible intended match
          517:  andl $-32, %eax 
check:38'0     ~~~~~~~~~~~~~~~~~
          518:  subl %ecx, %eax 
check:38'0     ~~~~~~~~~~~~~~~~~
          519:  shrl $2, %eax 
check:38'0     ~~~~~~~~~~~~~~~
          520:  retl 
check:38'0     ~~~~~~
          521: .Lfunc_end9: 
check:38'0     ~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
