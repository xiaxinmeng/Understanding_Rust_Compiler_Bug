asm
example::foo:
        mov     rax, rdi
        movups  xmm0, xmmword ptr [rsi]
        movups  xmmword ptr [rdi], xmm0
        ret
