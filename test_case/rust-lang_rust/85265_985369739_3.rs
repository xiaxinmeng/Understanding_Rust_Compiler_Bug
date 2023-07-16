asm
example::case_2:
        movups  xmm0, xmmword ptr [rdi]
        movups  xmm1, xmmword ptr [rsi]
        addps   xmm1, xmm0
        movups  xmmword ptr [rdx], xmm1
        ret
