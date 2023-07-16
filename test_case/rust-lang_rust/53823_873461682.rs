asm
.LBB0_4:
        mov     r9, rdx
        shr     r9
        lea     rax, [r9 + r8]
        cmp     qword ptr [rsi + 8*rax], rcx
        cmova   rax, r8
        sub     rdx, r9
        cmp     rdx, 1
        mov     r8, rax
        ja      .LBB0_4
