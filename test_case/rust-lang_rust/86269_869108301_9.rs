asm
; -O
example::lerp:
        movss   xmm3, dword ptr [rip + .LCPI0_0]
        subss   xmm3, xmm0
        mulss   xmm3, xmm1
        mulss   xmm0, xmm2
        addss   xmm0, xmm3
        cmpeqss xmm2, xmm1
        movaps  xmm3, xmm2
        andnps  xmm3, xmm0
        andps   xmm2, xmm1
        orps    xmm2, xmm3
        movaps  xmm0, xmm2
        ret

example::lerp_known:
        movss   xmm1, dword ptr [rip + .LCPI1_0]
        subss   xmm1, xmm0
        mulss   xmm1, dword ptr [rip + .LCPI1_1]
        mulss   xmm0, dword ptr [rip + .LCPI1_2]
        addss   xmm0, xmm1
        ret

; -O -target-cpu=skylake
example::lerp:
        vmovss  xmm3, dword ptr [rip + .LCPI0_0]
        vsubss  xmm3, xmm3, xmm0
        vmulss  xmm3, xmm3, xmm1
        vmulss  xmm0, xmm0, xmm2
        vaddss  xmm0, xmm3, xmm0
        vcmpeqss        xmm2, xmm1, xmm2
        vblendvps       xmm0, xmm0, xmm1, xmm2
        ret

example::lerp_known:
        vmovss  xmm1, dword ptr [rip + .LCPI1_0]
        vsubss  xmm1, xmm1, xmm0
        vmulss  xmm1, xmm1, dword ptr [rip + .LCPI1_1]
        vmulss  xmm0, xmm0, dword ptr [rip + .LCPI1_2]
        vaddss  xmm0, xmm0, xmm1
        ret
