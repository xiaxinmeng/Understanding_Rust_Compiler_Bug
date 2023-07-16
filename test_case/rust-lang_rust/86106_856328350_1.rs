asm

example::foo:
        mov     rax, rdi
        mov     rcx, qword ptr [rip + .L__unnamed_1]
        mov     qword ptr [rdi], rcx
        xorps   xmm0, xmm0
        movups  xmmword ptr [rdi + 8], xmm0
        ret

.L__unnamed_1:
        .asciz  "\001\000\000\000\000\000\000\000\000\000\000\000\000\000\000"


example::foo:
        mov     rax, rdi
        mov     qword ptr [rdi], 1
        xorps   xmm0, xmm0
        movups  xmmword ptr [rdi + 8], xmm0
        ret
