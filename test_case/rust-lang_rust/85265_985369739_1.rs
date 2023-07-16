asm
example::case_1:
        movq    xmm0, rcx
        movq    xmm1, rdx
        punpcklqdq      xmm1, xmm0
        movq    xmm0, rsi
        movq    xmm2, rdi
        punpcklqdq      xmm2, xmm0
        addps   xmm2, xmm1
        movups  xmmword ptr [r8], xmm2
        ret
