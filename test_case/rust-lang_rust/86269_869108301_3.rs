asm
; -O
example::lerp:
        push    rax
        movss   dword ptr [rsp + 4], xmm2
        movaps  xmm3, xmm1
        movss   dword ptr [rsp], xmm1
        movaps  xmm1, xmm2
        subss   xmm1, xmm3
        movaps  xmm2, xmm3
        call    qword ptr [rip + fmaf@GOTPCREL]
        movss   xmm2, dword ptr [rsp + 4]
        movss   xmm1, dword ptr [rsp]
        ucomiss xmm2, xmm1
        jae     .LBB0_3
        ucomiss xmm1, xmm2
        jb      .LBB0_5
        maxss   xmm2, xmm0
        minss   xmm1, xmm2
        jmp     .LBB0_4
.LBB0_3:
        maxss   xmm1, xmm0
        minss   xmm2, xmm1
        movaps  xmm1, xmm2
.LBB0_4:
        movaps  xmm0, xmm1
        pop     rax
        ret
.LBB0_5:
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 28
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2

example::lerp_known:
        push    rax
        movss   xmm1, dword ptr [rip + .LCPI1_0]
        movss   xmm2, dword ptr [rip + .LCPI1_1]
        call    qword ptr [rip + fmaf@GOTPCREL]
        movss   xmm1, dword ptr [rip + .LCPI1_1]
        maxss   xmm1, xmm0
        movss   xmm0, dword ptr [rip + .LCPI1_2]
        minss   xmm0, xmm1
        pop     rax
        ret

; -O -target-cpu=skylake
example::lerp:
        vsubss  xmm3, xmm2, xmm1
        vfmadd213ss     xmm3, xmm0, xmm1
        vucomiss        xmm2, xmm1
        jae     .LBB0_3
        vucomiss        xmm1, xmm2
        jb      .LBB0_5
        vmaxss  xmm0, xmm2, xmm3
        vminss  xmm0, xmm1, xmm0
        ret
.LBB0_3:
        vmaxss  xmm0, xmm1, xmm3
        vminss  xmm0, xmm2, xmm0
        ret
.LBB0_5:
        push    rax
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 28
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2

example::lerp_known:
        vmovss  xmm1, dword ptr [rip + .LCPI1_0]
        vfmadd132ss     xmm0, xmm1, dword ptr [rip + .LCPI1_1]
        vmaxss  xmm0, xmm1, xmm0
        vmovss  xmm1, dword ptr [rip + .LCPI1_2]
        vminss  xmm0, xmm1, xmm0
        ret
