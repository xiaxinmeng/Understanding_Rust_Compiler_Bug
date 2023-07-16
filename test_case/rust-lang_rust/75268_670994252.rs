asm
check::boundcheck:
 mov     rax, rdi
 test    rsi, rsi
 je      .LBB0_1
 mov     rdx, rsi
 cmp     byte, ptr, [rax], 95
 jne     .LBB0_4
 add     rax, 1
 add     rdx, -1
.LBB0_4:
 ret
.LBB0_1:
 xor     edx, edx
 ret
