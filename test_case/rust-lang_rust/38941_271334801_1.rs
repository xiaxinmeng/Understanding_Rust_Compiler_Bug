asm
example::foo:
        push    rbp
        mov     rbp, rsp
        mov     eax, dword ptr [rdx]
        mov     dword ptr [rdi], eax
        mov     dword ptr [rsi], eax
        pop     rbp
        ret
