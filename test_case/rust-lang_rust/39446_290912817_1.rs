asm
example::times_ten:
        imul    rax, rdi, 30
        movabs  rcx, 6148914691236517206
        imul    rcx
        mov     rax, rdx
        shr     rax, 63
        lea     rax, [rax + rdx]
        ret
