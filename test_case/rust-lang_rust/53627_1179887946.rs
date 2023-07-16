asm
example::add:
        xor     ecx, ecx
        xor     eax, eax
.LBB0_1:
        cmp     rsi, rcx
        je      .LBB0_2
        add     eax, dword ptr [rdi + 4*rcx]
        inc     rcx
        jmp     .LBB0_1
.LBB0_2:
        ret
