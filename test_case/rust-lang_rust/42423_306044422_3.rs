asm
clamp:
        sub     rsp, 8
        vxorpd  xmm1, xmm1, xmm1
        vcvtss2sd       xmm0, xmm0, xmm0
        call    fmax
        vmovsd  xmm1, QWORD PTR .LC1[rip]
        call    fmin
        add     rsp, 8
        vcvtsd2ss       xmm0, xmm0, xmm0
        ret
