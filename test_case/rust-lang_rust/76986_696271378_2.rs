asm
example::sum_c:
        mov     eax, dword ptr [rsi]
        add     eax, dword ptr [rdi]
        mov     ecx, dword ptr [rsi + 4]
        add     ecx, dword ptr [rdi + 4]
        mov     edx, dword ptr [rsi + 8]
        add     edx, dword ptr [rdi + 8]
        shl     rcx, 32
        or      rax, rcx
        ret
