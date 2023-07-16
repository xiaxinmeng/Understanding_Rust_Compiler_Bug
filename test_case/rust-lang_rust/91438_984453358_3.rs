asm
example::run:
        mov     rdx, rsi
        mov     rax, qword ptr [rdx + 16]
        test    rax, rax
        mov     ecx, 1
        cmove   rax, rcx
        add     rax, rdi
        ret
