plain
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....................................iii..............F..
failures:

---- [assembly] src/test/assembly/x86_64-floating-point-clamp.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s" "/checkout/src/test/assembly/x86_64-floating-point-clamp.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/x86_64-floating-point-clamp.rs:19:17: error: CHECK-NEXT: is not on the line after the previous match
 // CHECK-NEXT: movss xmm1
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:40:2: note: 'next' match was here
 movss xmm1, dword ptr [rip + .LCPI1_0]
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:38:14: note: previous match ended here
clamp12_demo:
             ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:39:1: note: non-matching line after previous match is here
 .cfi_startproc


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s
Check file: /checkout/src/test/assembly/x86_64-floating-point-clamp.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
        35:  .globl clamp12_demo 
        36:  .p2align 4, 0x90 
        37:  .type clamp12_demo,@function 
        38: clamp12_demo: 
        39:  .cfi_startproc 
        40:  movss xmm1, dword ptr [rip + .LCPI1_0] 
next:19      !~~~~~~~~~                              error: match on wrong line
        41:  maxss xmm1, xmm0 
        42:  movss xmm0, dword ptr [rip + .LCPI1_1] 
        43:  minss xmm0, xmm1 
        44:  ret 
        45: .Lfunc_end1: 
         .
         .
>>>>>>
------------------------------------------
------------------------------------------



failures:
    [assembly] src/test/assembly/x86_64-floating-point-clamp.rs
test result: FAILED. 126 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.49s

Build completed unsuccessfully in 0:10:16
