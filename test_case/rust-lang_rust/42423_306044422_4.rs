asm
clamp:
        vcvtss2sd       xmm0, xmm0, xmm0
        vmaxsd  xmm0, xmm0, QWORD PTR .LC0[rip]
        vminsd  xmm0, xmm0, QWORD PTR .LC1[rip]
        vcvtsd2ss       xmm0, xmm0, xmm0
        ret
