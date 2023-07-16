asm
example::sum:
        movss   xmm0, dword ptr [rdi]
        addss   xmm0, dword ptr [rsi]
        movss   dword ptr [rdx], xmm0
        movss   xmm0, dword ptr [rdi + 4]
        addss   xmm0, dword ptr [rsi + 4]
        movss   dword ptr [rdx + 4], xmm0
        movss   xmm0, dword ptr [rdi + 8]
        addss   xmm0, dword ptr [rsi + 8]
        movss   dword ptr [rdx + 8], xmm0
        ret
