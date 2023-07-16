plain
failures:

---- [assembly] src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs#strong stdout ----

error in revision `strong`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s" "/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs" "--check-prefixes" "CHECK,NONMSVC,strong"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs:373:17: error: CHECK-NOT: excluded string found in input
 // strong-NOT: __stack_chk_fail
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s:621:8: note: found here
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
       616:  ud2
       617: .LBB17_10:
       618:  ud2
       619:  ud2
       620: .LBB17_7:
       621:  callq __stack_chk_fail@PLT
not:373            !~~~~~~~~~~~~~~~     error: no match expected
       622:  ud2
       623: .LBB17_6:
       624: .Ltmp8:
       625:  movq %rax, %rbx
       626:  movq %rsp, %rdi
         .
         .
>>>>>>
------------------------------------------
