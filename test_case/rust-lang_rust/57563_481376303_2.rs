asm
example::return_empty_vec:
        mov     rax, rdi
        mov     qword ptr [rdi], 8
        xorps   xmm0, xmm0
        movups  xmmword ptr [rdi + 8], xmm0
        ret
