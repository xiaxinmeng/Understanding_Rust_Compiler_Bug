asm
example::make:
        mov     rax, rdi
        mov     qword ptr [rdi], rsi
        mov     byte ptr [rdi + 8], 1
        mov     dword ptr [rdi + 9], 0
        mov     dword ptr [rdi + 12], 0
        ret

example::make_constant:
        mov     rax, rdi
        mov     qword ptr [rdi], 5
        movabs  rcx, 4294967296
        mov     qword ptr [rdi + 8], rcx
        ret
