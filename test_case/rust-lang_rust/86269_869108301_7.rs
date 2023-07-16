asm
; -O
example::lerp:
        push    rax
        movss   xmm3, dword ptr [rip + .LCPI0_0]
        subss   xmm3, xmm0
        mulss   xmm3, xmm1
        mulss   xmm0, xmm2
        addss   xmm0, xmm3
        ucomiss xmm2, xmm1
        jbe     .LBB0_1
        jb      .LBB0_6
        maxss   xmm1, xmm0
        minss   xmm2, xmm1
        movaps  xmm1, xmm2
        jmp     .LBB0_5
.LBB0_1:
        ucomiss xmm1, xmm2
        jb      .LBB0_6
        maxss   xmm2, xmm0
        minss   xmm1, xmm2
.LBB0_5:
        movaps  xmm0, xmm1
        pop     rax
        ret
.LBB0_6:
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 28
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2

example::lerp_known:
        movss   xmm2, dword ptr [rip + .LCPI1_0]
        subss   xmm2, xmm0
        movss   xmm3, dword ptr [rip + .LCPI1_1]
        mulss   xmm2, xmm3
        movss   xmm1, dword ptr [rip + .LCPI1_2]
        mulss   xmm0, xmm1
        addss   xmm0, xmm2
        maxss   xmm3, xmm0
        minss   xmm1, xmm3
        movaps  xmm0, xmm1
        ret

; -O -target-cpu=skylake
example::lerp:
        push    rax
        vmovss  xmm3, dword ptr [rip + .LCPI0_0]
        vsubss  xmm3, xmm3, xmm0
        vmulss  xmm3, xmm3, xmm1
        vmulss  xmm0, xmm0, xmm2
        vaddss  xmm0, xmm3, xmm0
        vucomiss        xmm2, xmm1
        jbe     .LBB0_1
        jb      .LBB0_6
        vmaxss  xmm0, xmm1, xmm0
        vminss  xmm0, xmm2, xmm0
        pop     rax
        ret
.LBB0_1:
        vucomiss        xmm1, xmm2
        jb      .LBB0_6
        vmaxss  xmm0, xmm2, xmm0
        vminss  xmm0, xmm1, xmm0
        pop     rax
        ret
.LBB0_6:
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 28
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2

example::lerp_known:
        vmovss  xmm1, dword ptr [rip + .LCPI1_0]
        vsubss  xmm1, xmm1, xmm0
        vmovss  xmm2, dword ptr [rip + .LCPI1_1]
        vmulss  xmm1, xmm1, xmm2
        vmovss  xmm3, dword ptr [rip + .LCPI1_2]
        vmulss  xmm0, xmm0, xmm3
        vaddss  xmm0, xmm0, xmm1
        vmaxss  xmm0, xmm2, xmm0
        vminss  xmm0, xmm3, xmm0
        ret
