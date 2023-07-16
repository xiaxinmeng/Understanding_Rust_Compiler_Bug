asm
; -O
example::lerp:
        xorps   xmm3, xmm3
        ucomiss xmm3, xmm1
        jb      .LBB0_2
        ucomiss xmm2, xmm3
        jae     .LBB0_9
.LBB0_2:
        ucomiss xmm1, xmm3
        jb      .LBB0_4
        xorps   xmm3, xmm3
        ucomiss xmm3, xmm2
        jb      .LBB0_4
.LBB0_9:
        mulss   xmm2, xmm0
        movss   xmm3, dword ptr [rip + .LCPI0_0]
        subss   xmm3, xmm0
        mulss   xmm3, xmm1
        addss   xmm2, xmm3
.LBB0_10:
        movaps  xmm0, xmm2
        ret
.LBB0_4:
        ucomiss xmm0, dword ptr [rip + .LCPI0_0]
        jne     .LBB0_5
        jnp     .LBB0_10
.LBB0_5:
        movaps  xmm3, xmm2
        subss   xmm3, xmm1
        mulss   xmm3, xmm0
        addss   xmm3, xmm1
        ucomiss xmm0, dword ptr [rip + .LCPI0_0]
        jbe     .LBB0_7
        ucomiss xmm2, xmm1
        jbe     .LBB0_7
        movaps  xmm0, xmm2
        cmpunordss      xmm0, xmm2
        movaps  xmm1, xmm0
        andps   xmm1, xmm3
        maxss   xmm3, xmm2
        jmp     .LBB0_8
.LBB0_7:
        movaps  xmm0, xmm2
        cmpunordss      xmm0, xmm2
        movaps  xmm1, xmm0
        andps   xmm1, xmm3
        minss   xmm3, xmm2
.LBB0_8:
        andnps  xmm0, xmm3
        orps    xmm0, xmm1
        ret

example::lerp_known:
        ucomiss xmm0, dword ptr [rip + .LCPI1_0]
        jne     .LBB1_1
        jp      .LBB1_1
        movss   xmm0, dword ptr [rip + .LCPI1_3]
        ret
.LBB1_1:
        ucomiss xmm0, dword ptr [rip + .LCPI1_0]
        mulss   xmm0, dword ptr [rip + .LCPI1_1]
        addss   xmm0, dword ptr [rip + .LCPI1_2]
        jbe     .LBB1_4
        maxss   xmm0, dword ptr [rip + .LCPI1_3]
        ret
.LBB1_4:
        minss   xmm0, dword ptr [rip + .LCPI1_3]
        ret

; -O -target-cpu=skylake
example::lerp:
        vxorps  xmm3, xmm3, xmm3
        vucomiss        xmm3, xmm1
        jb      .LBB0_2
        vucomiss        xmm2, xmm3
        jae     .LBB0_9
.LBB0_2:
        vucomiss        xmm1, xmm3
        jb      .LBB0_4
        vxorps  xmm3, xmm3, xmm3
        vucomiss        xmm3, xmm2
        jb      .LBB0_4
.LBB0_9:
        vmulss  xmm2, xmm0, xmm2
        vmovss  xmm3, dword ptr [rip + .LCPI0_0]
        vsubss  xmm0, xmm3, xmm0
        vmulss  xmm0, xmm0, xmm1
        vaddss  xmm2, xmm0, xmm2
.LBB0_10:
        vmovaps xmm0, xmm2
        ret
.LBB0_4:
        vucomiss        xmm0, dword ptr [rip + .LCPI0_0]
        jne     .LBB0_5
        jnp     .LBB0_10
.LBB0_5:
        vsubss  xmm3, xmm2, xmm1
        vmulss  xmm3, xmm3, xmm0
        vaddss  xmm3, xmm3, xmm1
        vucomiss        xmm0, dword ptr [rip + .LCPI0_0]
        jbe     .LBB0_7
        vucomiss        xmm2, xmm1
        jbe     .LBB0_7
        vmaxss  xmm0, xmm3, xmm2
        jmp     .LBB0_8
.LBB0_7:
        vminss  xmm0, xmm3, xmm2
.LBB0_8:
        vcmpunordss     xmm1, xmm2, xmm2
        vblendvps       xmm0, xmm0, xmm3, xmm1
        ret

example::lerp_known:
        vucomiss        xmm0, dword ptr [rip + .LCPI1_0]
        jne     .LBB1_1
        jp      .LBB1_1
        vmovss  xmm0, dword ptr [rip + .LCPI1_3]
        ret
.LBB1_1:
        vmulss  xmm1, xmm0, dword ptr [rip + .LCPI1_1]
        vaddss  xmm1, xmm1, dword ptr [rip + .LCPI1_2]
        vucomiss        xmm0, dword ptr [rip + .LCPI1_0]
        jbe     .LBB1_4
        vmaxss  xmm0, xmm1, dword ptr [rip + .LCPI1_3]
        ret
.LBB1_4:
        vminss  xmm0, xmm1, dword ptr [rip + .LCPI1_3]
        ret
