asm
foo:                                    # @foo
# BB#0:
        movq    xmm0, rcx
        movq    xmm1, rdx
        punpcklqdq      xmm1, xmm0      # xmm1 = xmm1[0],xmm0[0]
        movq    xmm0, rsi
        movq    xmm2, rdi
        punpcklqdq      xmm2, xmm0      # xmm2 = xmm2[0],xmm0[0]
        pcmpeqb xmm2, xmm1
        pmovmskb        eax, xmm2
        cmp     eax, 65535
        sete    al
        ret
                                        # -- End function
