plain
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#strong stdout ----

error in revision `strong`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s" "/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs" "--check-prefixes" "CHECK,NONMSVC,strong"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs:373:17: error: CHECK-NOT: excluded string found in input
 // strong-NOT: __stack_chk_fail
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s:606:8: note: found here
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
       601:  ud2
       602: .LBB19_10:
       603:  ud2
       604:  ud2
       605: .LBB19_7:
       606:  callq __stack_chk_fail@PLT
not:373            !~~~~~~~~~~~~~~~     error: no match expected
       607:  ud2
       608: .LBB19_6:
       609: .Ltmp5:
       610:  movq %rax, %r14
       611:  movq %r15, %rdi
         .
         .
>>>>>>
------------------------------------------
