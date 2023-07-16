asm
example::typed_copy:
        mov     al, byte ptr [rdi]
        mov     byte ptr [rsi], al
        ret

example::untyped_copy:
        movaps  xmm0, xmmword ptr [rdi + 112]
        movaps  xmmword ptr [rsi + 112], xmm0
        movaps  xmm0, xmmword ptr [rdi + 96]
        movaps  xmmword ptr [rsi + 96], xmm0
        movaps  xmm0, xmmword ptr [rdi + 80]
        movaps  xmmword ptr [rsi + 80], xmm0
        movaps  xmm0, xmmword ptr [rdi + 64]
        movaps  xmmword ptr [rsi + 64], xmm0
        movaps  xmm0, xmmword ptr [rdi]
        movaps  xmm1, xmmword ptr [rdi + 16]
        movaps  xmm2, xmmword ptr [rdi + 32]
        movaps  xmm3, xmmword ptr [rdi + 48]
        movaps  xmmword ptr [rsi + 48], xmm3
        movaps  xmmword ptr [rsi + 32], xmm2
        movaps  xmmword ptr [rsi + 16], xmm1
        movaps  xmmword ptr [rsi], xmm0
        ret
