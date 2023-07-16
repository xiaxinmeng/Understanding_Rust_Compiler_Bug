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
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s:617:8: note: found here
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
       612:  ud2 
       613: .LBB17_8: 
       614:  ud2 
       615:  ud2 
       616: .LBB17_5: 
       617:  callq __stack_chk_fail@PLT 
not:373            !~~~~~~~~~~~~~~~      error: no match expected
       618:  ud2 
       619: .LBB17_9: 
       620: .Ltmp8: 
       621:  movq %rax, %rbx 
       622:  movq %rsp, %rdi 
         .
         .
>>>>>>
------------------------------------------
