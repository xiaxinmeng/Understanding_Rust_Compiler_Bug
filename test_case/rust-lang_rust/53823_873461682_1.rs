asm
.LBB0_3:
        shr     rdx
        add     rdx, rsi
        cmp     qword ptr [rdi + 8*rdx], r8
        jb      .LBB0_6
        je      .LBB0_9
        mov     rcx, rdx
        sub     rdx, rsi
        ja      .LBB0_3
