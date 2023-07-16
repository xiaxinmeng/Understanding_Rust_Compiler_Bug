asm
example::foo:
        mov     eax, edi
        ret

example::bar:
        mov     rax, rdi
        mov     dword ptr [rdi], esi
        mov     byte ptr [rdi + 52], 0
        ret
