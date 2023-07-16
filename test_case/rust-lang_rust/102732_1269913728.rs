plain
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [assembly] src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs#strong stdout ----

error in revision `strong`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s" "/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,strong"
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
       616:  movq $0, 40(%rsp) 
       617:  movq %rsp, %rdi 
       618:  callq *_ZN4core9panicking18panic_fmt_nounwind17hab609893b7434be9E@GOTPCREL(%rip) 
       619:  ud2 
       620: .LBB17_5: 
       621:  callq __stack_chk_fail@PLT 
not:373            !~~~~~~~~~~~~~~~      error: no match expected
       622:  ud2 
       623: .LBB17_9: 
       624: .Ltmp8: 
       625:  movq %rax, %r14 
       626:  movq %rbx, %rdi 
         .
         .
>>>>>>
------------------------------------------
