asm
example::run:
        mov     rdx, rsi
        mov     rax, qword ptr [rdx + 16]
        add     rax, rdi
        ret
