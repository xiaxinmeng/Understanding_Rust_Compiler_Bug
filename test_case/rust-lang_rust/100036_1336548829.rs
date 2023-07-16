plain
failures:

---- [assembly] src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs#strong stdout ----

error in revision `strong`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s" "/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,strong"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs:373:17: error: CHECK-NOT: excluded string found in input
 // strong-NOT: __stack_chk_fail
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s:619:8: note: found here
 callq __stack_chk_fail@PLT

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s
Check file: /checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
       614:  leaq .L__unnamed_5(%rip), %rdi 
       615:  movl $166, %esi 
       616:  callq *_ZN4core9panicking18panic_str_nounwind17he853254ad2d945b0E@GOTPCREL(%rip) 
       617:  ud2 
       618: .LBB17_5: 
       619:  callq __stack_chk_fail@PLT 
not:373            !~~~~~~~~~~~~~~~      error: no match expected
       620:  ud2 
       621: .LBB17_9: 
       622: .Ltmp8: 
       623:  movq %rax, %rbx 
       624:  movq %rsp, %rdi 
         .
         .
>>>>>>
------------------------------------------
