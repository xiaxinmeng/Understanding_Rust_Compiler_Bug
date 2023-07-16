asm
example::adds:
        mov     eax, dword ptr [rsi]
        add     eax, eax
        add     dword ptr [rdi], eax
        ret
