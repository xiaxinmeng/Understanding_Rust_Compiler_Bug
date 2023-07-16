assembly
example::f:
        sub     rsp, 24
        mov     rax, qword ptr [rip + core::hint::black_box@GOTPCREL]
        mov     qword ptr [rsp], rax
        movsd   xmm0, qword ptr [rip + .LCPI7_0]
        movsd   qword ptr [rsp + 8], xmm0
        call    rax
        mov     rax, qword ptr [rsp]
        movaps  xmm1, xmm0
        movsd   xmm0, qword ptr [rsp + 8]
        movsd   qword ptr [rsp + 16], xmm1
        call    rax
        movaps  xmm1, xmm0
        movsd   xmm0, qword ptr [rsp + 16]
        mov     rax, qword ptr [rip + fmod@GOTPCREL]
        call    rax
        add     rsp, 24
        ret

example::g:
        xorps   xmm0, xmm0
        ret
