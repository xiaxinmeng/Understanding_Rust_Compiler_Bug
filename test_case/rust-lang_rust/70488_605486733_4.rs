asm
example::foo:
        mov     eax, edi
        ret

example::bar:
        mov     rax, rdi
        mov     dword ptr [rdi], esi
        mov     dword ptr [rdi + 4], 0
        ret
