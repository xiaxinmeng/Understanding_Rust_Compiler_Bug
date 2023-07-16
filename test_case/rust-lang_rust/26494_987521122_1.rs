asm
example::case_1:
        mov     rax, rdi
        movups  xmm0, xmmword ptr [rsi]
        movups  xmm1, xmmword ptr [rdx]
        addps   xmm1, xmm0
        movups  xmmword ptr [rdi], xmm1
        ret
