asm
 push    r15
 push    r14
 push    rbx
 cmp     rdi, rsi
 jae     .LBB0_3
 mov     r14, rsi
 mov     rbx, rdi
 mov     r15, qword, ptr, [rip, +, f@GOTPCREL]
.LBB0_2:
 mov     rdi, rbx
 call    r15
 add     rbx, 4
 cmp     rbx, r14
 jb      .LBB0_2
.LBB0_3:
 pop     rbx
 pop     r14
 pop     r15
 ret
