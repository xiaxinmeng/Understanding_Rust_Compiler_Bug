asm
clamp:                                  # @clamp
        vxorps  xmm1, xmm1, xmm1
        vmaxss  xmm1, xmm1, xmm0
        vcmpunordss     xmm0, xmm0, xmm0
        vandnps xmm0, xmm0, xmm1
        vmovss  xmm1, dword ptr [rip + .LCPI0_0] # xmm1 = mem[0],zero,zero,zero
        vminss  xmm2, xmm1, xmm0
        vcmpunordss     xmm0, xmm0, xmm0
        vandnps xmm2, xmm0, xmm2
        vandps  xmm0, xmm0, xmm1
        vorps   xmm0, xmm2, xmm0
        ret
