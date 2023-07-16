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
        jb      .LBB1_2
        minss   xmm2, xmm0
        cmpnless        xmm0, xmm1
        andps   xmm2, xmm0
        andnps  xmm0, xmm1
        orps    xmm0, xmm2
        ret
.LBB1_2:
        push    rax
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_3]
        mov     esi, 28
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2
