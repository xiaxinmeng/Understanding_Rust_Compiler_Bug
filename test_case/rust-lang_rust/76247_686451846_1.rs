asm
.LBB0_6:
        movq    xmm1, qword ptr [r8 + rsi]
        movq    xmm2, qword ptr [r8 + rsi + 8]
        movdqu  xmm3, xmmword ptr [r10 + 2*rsi]
        movdqu  xmm4, xmmword ptr [r10 + 2*rsi + 16]
        movdqu  xmm5, xmmword ptr [rdx + 2*rsi]
        pcmpgtw xmm5, xmm3
        packsswb        xmm5, xmm5
        movdqu  xmm3, xmmword ptr [rdx + 2*rsi + 16]
        pcmpgtw xmm3, xmm4
        packsswb        xmm3, xmm3
        pand    xmm1, xmm0
        pand    xmm1, xmm5
        pand    xmm2, xmm0
        pand    xmm2, xmm3
        movq    qword ptr [r8 + rsi], xmm1
        movq    qword ptr [r8 + rsi + 8], xmm2
        add     rsi, 16
        cmp     rcx, rsi
        jne     .LBB0_6
