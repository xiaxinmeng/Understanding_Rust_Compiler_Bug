asm
clamp:                                  # @clamp
        vxorps  xmm1, xmm1, xmm1
        vmaxss  xmm0, xmm0, xmm1
        vminss  xmm0, xmm0, dword ptr [rip + .LCPI0_0]
        ret
