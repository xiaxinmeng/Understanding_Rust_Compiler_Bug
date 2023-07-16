asm
sum(Stats const&, Stats const&):                      # @sum(Stats const&, Stats const&)
        movss   xmm0, dword ptr [rdi]           # xmm0 = mem[0],zero,zero,zero
        movss   xmm1, dword ptr [rdi + 4]       # xmm1 = mem[0],zero,zero,zero
        addss   xmm0, dword ptr [rsi]
        addss   xmm1, dword ptr [rsi + 4]
        unpcklps        xmm0, xmm1                      # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
        movss   xmm1, dword ptr [rdi + 8]       # xmm1 = mem[0],zero,zero,zero
        addss   xmm1, dword ptr [rsi + 8]
        ret
