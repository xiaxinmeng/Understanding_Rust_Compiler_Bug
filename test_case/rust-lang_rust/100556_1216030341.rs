asm
example::clamp_old:
        ucomiss xmm2, xmm1
        jb      .LBB0_2
        maxss   xmm1, xmm0
        minss   xmm2, xmm1
        movaps  xmm0, xmm2
        ret
.LBB0_2:
        push    rax
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 28
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2

example::clamp_new:
        ucomiss xmm2, xmm1
        jb      .LBB1_5
        ucomiss xmm1, xmm0
        jae     .LBB1_4
        ucomiss xmm0, xmm2
        movaps  xmm1, xmm0
        jbe     .LBB1_4
        movaps  xmm1, xmm2
.LBB1_4:
        movaps  xmm0, xmm1
        ret
.LBB1_5:
        push    rax
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_3]
        mov     esi, 28
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2
