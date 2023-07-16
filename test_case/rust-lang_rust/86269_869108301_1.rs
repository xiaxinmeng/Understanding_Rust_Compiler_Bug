asm
; -O
example::lerp:
        movaps  xmm3, xmm1
        subss   xmm2, xmm1
        movaps  xmm1, xmm2
        movaps  xmm2, xmm3
        jmp     qword ptr [rip + fmaf@GOTPCREL]

example::lerp_known:
        movss   xmm1, dword ptr [rip + .LCPI1_0]
        movss   xmm2, dword ptr [rip + .LCPI1_1]
        jmp     qword ptr [rip + fmaf@GOTPCREL]

; -O -target-cpu=skylake
example::lerp:
        vsubss  xmm2, xmm2, xmm1
        vfmadd213ss     xmm0, xmm2, xmm1
        ret

example::lerp_known:
        vmovss  xmm1, dword ptr [rip + .LCPI1_0]
        vfmadd213ss     xmm0, xmm1, dword ptr [rip + .LCPI1_1]
        ret
