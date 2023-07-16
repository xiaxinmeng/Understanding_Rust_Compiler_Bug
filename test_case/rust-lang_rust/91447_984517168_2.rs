asm
example::sum:
        movss   xmm0, dword ptr [rdi]
        addss   xmm0, dword ptr [rsi]
        movsd   xmm1, qword ptr [rdi + 4]
        movsd   xmm2, qword ptr [rsi + 4]
        addps   xmm2, xmm1
        movd    eax, xmm0
        movd    ecx, xmm2
        shufps  xmm2, xmm2, 85
        movd    edx, xmm2
        shl     rcx, 32
        or      rax, rcx
        ret
