asm
        mov     eax, dword ptr [rdi]
        mov     dword ptr [rsp + 4], eax
        lea     rax, [rsp + 4]
