asm
example::xor_in_place:
        push    rbp
        mov     rbp, rsp
        movups  xmm0, xmmword ptr [rsi]
        movups  xmm1, xmmword ptr [rdi]
        xorps   xmm1, xmm0
        movups  xmmword ptr [rdi], xmm1
        pop     rbp
        ret

example::xor_in_place2:
        push    rbp
        mov     rbp, rsp
        movups  xmm0, xmmword ptr [rsi]
        movups  xmm1, xmmword ptr [rdi]
        xorps   xmm1, xmm0
        movups  xmmword ptr [rdi], xmm1
        pop     rbp
        ret
