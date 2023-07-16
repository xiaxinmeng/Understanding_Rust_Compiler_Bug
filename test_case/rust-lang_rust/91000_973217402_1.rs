asm
 push    r15
 push    r14
 push    rbx
 cmp     rdi, rsi
 jae     .LBB0_3
 mov     r15, rsi
 mov     r14, qword, ptr, [rip, +, f@GOTPCREL]
.LBB0_2:
 mov     rax, r15
 sub     rax, rdi
 lea     rbx, [rdi, +, 4]
 cmp     rax, 4
 cmovb   rbx, r15
 call    r14
 mov     rdi, rbx
 cmp     rbx, r15
 jb      .LBB0_2
.LBB0_3:
 pop     rbx
 pop     r14
 pop     r15
 ret
