asm
example::sum_f32_big_8:
        mov     rax, rdi
        movss   xmm0, dword ptr [rsi]
        addss   xmm0, dword ptr [rdx]
        movss   dword ptr [rdi], xmm0
        movss   xmm0, dword ptr [rsi + 4]
        addss   xmm0, dword ptr [rdx + 4]
        movss   dword ptr [rdi + 4], xmm0
        movss   xmm0, dword ptr [rsi + 8]
        addss   xmm0, dword ptr [rdx + 8]
        movss   dword ptr [rdi + 8], xmm0
        movss   xmm0, dword ptr [rsi + 12]
        addss   xmm0, dword ptr [rdx + 12]
        movss   dword ptr [rdi + 12], xmm0
        movss   xmm0, dword ptr [rsi + 16]
        addss   xmm0, dword ptr [rdx + 16]
        movss   dword ptr [rdi + 16], xmm0
        movss   xmm0, dword ptr [rsi + 20]
        addss   xmm0, dword ptr [rdx + 20]
        movss   dword ptr [rdi + 20], xmm0
        movss   xmm0, dword ptr [rsi + 24]
        addss   xmm0, dword ptr [rdx + 24]
        movss   dword ptr [rdi + 24], xmm0
        movss   xmm0, dword ptr [rsi + 28]
        addss   xmm0, dword ptr [rdx + 28]
        movss   dword ptr [rdi + 28], xmm0
        ret

example::sum_f32_big_16:
        mov     rax, rdi
        movss   xmm0, dword ptr [rsi]
        addss   xmm0, dword ptr [rdx]
        movss   dword ptr [rdi], xmm0
        movss   xmm0, dword ptr [rsi + 4]
        addss   xmm0, dword ptr [rdx + 4]
        movss   dword ptr [rdi + 4], xmm0
        movss   xmm0, dword ptr [rsi + 8]
        addss   xmm0, dword ptr [rdx + 8]
        movss   dword ptr [rdi + 8], xmm0
        movss   xmm0, dword ptr [rsi + 12]
        addss   xmm0, dword ptr [rdx + 12]
        movss   dword ptr [rdi + 12], xmm0
        movss   xmm0, dword ptr [rsi + 16]
        addss   xmm0, dword ptr [rdx + 16]
        movss   dword ptr [rdi + 16], xmm0
        movss   xmm0, dword ptr [rsi + 20]
        addss   xmm0, dword ptr [rdx + 20]
        movss   dword ptr [rdi + 20], xmm0
        movss   xmm0, dword ptr [rsi + 24]
        addss   xmm0, dword ptr [rdx + 24]
        movss   dword ptr [rdi + 24], xmm0
        movss   xmm0, dword ptr [rsi + 28]
        addss   xmm0, dword ptr [rdx + 28]
        movss   dword ptr [rdi + 28], xmm0
        movss   xmm0, dword ptr [rsi + 32]
        addss   xmm0, dword ptr [rdx + 32]
        movss   dword ptr [rdi + 32], xmm0
        movss   xmm0, dword ptr [rsi + 36]
        addss   xmm0, dword ptr [rdx + 36]
        movss   dword ptr [rdi + 36], xmm0
        movss   xmm0, dword ptr [rsi + 40]
        addss   xmm0, dword ptr [rdx + 40]
        movss   dword ptr [rdi + 40], xmm0
        movss   xmm0, dword ptr [rsi + 44]
        addss   xmm0, dword ptr [rdx + 44]
        movss   dword ptr [rdi + 44], xmm0
        movss   xmm0, dword ptr [rsi + 48]
        addss   xmm0, dword ptr [rdx + 48]
        movss   dword ptr [rdi + 48], xmm0
        movss   xmm0, dword ptr [rsi + 52]
        addss   xmm0, dword ptr [rdx + 52]
        movss   dword ptr [rdi + 52], xmm0
        movss   xmm0, dword ptr [rsi + 56]
        addss   xmm0, dword ptr [rdx + 56]
        movss   dword ptr [rdi + 56], xmm0
        movss   xmm0, dword ptr [rsi + 60]
        addss   xmm0, dword ptr [rdx + 60]
        movss   dword ptr [rdi + 60], xmm0
        ret
