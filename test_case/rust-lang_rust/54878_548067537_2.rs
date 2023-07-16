asm
example::adds:
        mov     eax, dword ptr [rdi]
        add     eax, dword ptr [rsi]
        mov     dword ptr [rdi], eax
        add     eax, dword ptr [rsi]
        mov     dword ptr [rdi], eax
        ret
