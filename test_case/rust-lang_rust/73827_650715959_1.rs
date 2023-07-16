asm
get4:
        or      rsi, rdx
        cmp     rsi, 7
        ja      .LBB1_1
        mov     al, byte ptr [rdi + rdx]
        ret
.LBB1_1:
        xor     eax, eax
        ret
