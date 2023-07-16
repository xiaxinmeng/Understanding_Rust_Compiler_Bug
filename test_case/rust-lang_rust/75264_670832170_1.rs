asm
example::dynamic_fast_alt:
        mov     rax, rdx
        mov     rcx, rsi
        neg     rcx
        xor     edx, edx
        cmp     rcx, rax
        cmovae  rdx, rsi
        ret
