asm
example::case_1:
        movss   xmm0, dword ptr [rdi]
        movss   xmm1, dword ptr [rdi + 4]
        addss   xmm0, dword ptr [rsi]
        addss   xmm1, dword ptr [rsi + 4]
        movsd   xmm2, qword ptr [rdi + 8]
        movsd   xmm3, qword ptr [rsi + 8]
        addps   xmm3, xmm2
        movd    eax, xmm0
        movd    ecx, xmm1
        movd    esi, xmm3
        shufps  xmm3, xmm3, 229
        movd    edx, xmm3
        shl     rdx, 32
        or      rdx, rsi
        shl     rcx, 32
        or      rax, rcx
        ret
