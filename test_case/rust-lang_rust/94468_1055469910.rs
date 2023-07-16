plain
 finished in 0.436 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 138 tests
iiiiiii..Fiiiiiiiiii................................................................................ 100/138
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [assembly] assembly/asm/global_asm.rs stdout ----
---- [assembly] assembly/asm/global_asm.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/global_asm/global_asm.s" "/checkout/src/test/assembly/asm/global_asm.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/asm/global_asm.rs:24:11: error: CHECK: expected string not found in input
// CHECK: lea rax, [rip + MY_STATIC]
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/global_asm/global_asm.s:16:2: note: scanning from here
 lea rax, [rip + __unnamed_1]

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/global_asm/global_asm.s
Check file: /checkout/src/test/assembly/asm/global_asm.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
          .
          .
          .
          .
         11:  mov ecx, 5
         12: 
         13:  call my_func
         15: 
         15: 
         16:  lea rax, [rip + __unnamed_1]
check:24      X~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:24     ~
         18: 
check:24     ~
         19: 
         19: 
check:24     ~
         20:  .section .text.my_func,"ax",@progbits
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         21:  .globl my_func
          .
          .
          .
>>>>>>
