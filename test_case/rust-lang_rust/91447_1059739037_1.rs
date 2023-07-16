asm
sum(Stats const&, Stats const&):                      # @sum(Stats const&, Stats const&)
        movsd   xmm1, qword ptr [rdi]           # xmm1 = mem[0],zero
        movsd   xmm0, qword ptr [rsi]           # xmm0 = mem[0],zero
        addps   xmm0, xmm1
        movss   xmm1, dword ptr [rdi + 8]       # xmm1 = mem[0],zero,zero,zero
        addss   xmm1, dword ptr [rsi + 8]
        ret
