asm
example::foo:
        mov     rax, qword ptr [rdi + 16]
        xor     ecx, ecx
        sub     rax, 1
        cmovae  rcx, rax
        mov     qword ptr [rdi + 16], rcx
        ret
