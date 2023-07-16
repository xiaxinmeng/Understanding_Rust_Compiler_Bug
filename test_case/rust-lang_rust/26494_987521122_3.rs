asm
example::case_1:
        movd    xmm0, esi
        shr     rsi, 32
        movd    xmm1, edi
        shr     rdi, 32
        movd    xmm2, edi
        punpckldq       xmm1, xmm2
        movd    xmm2, esi
        movd    xmm3, edx
        shr     rdx, 32
        movd    xmm4, edx
        punpckldq       xmm3, xmm4
        movd    xmm4, ecx
        shr     rcx, 32
        addps   xmm3, xmm1
        addss   xmm4, xmm0
        movd    xmm0, ecx
        addss   xmm0, xmm2
        movd    edx, xmm4
        movd    eax, xmm0
        shl     rax, 32
        or      rdx, rax
        movd    ecx, xmm3
        shufps  xmm3, xmm3, 229
        movd    eax, xmm3
        shl     rax, 32
        or      rax, rcx
        ret
