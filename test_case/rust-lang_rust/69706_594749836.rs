asm
example::last:
        xor     ecx, ecx
        mov     rdx, rsi
        sub     rdx, 1
        lea     rax, [rdi + 4*rdx]
        cmovb   rax, rcx
        cmp     rdx, rsi
        cmovae  rax, rcx
        ret

example::last_pattern:
        xor     eax, eax
        sub     rsi, 1
        lea     rcx, [rdi + 4*rsi]
        cmovae  rax, rcx
        ret
