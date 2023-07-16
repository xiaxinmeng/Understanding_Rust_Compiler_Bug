asm
.LCPI0_0:
        .long   42
        .long   0
        .long   0
        .long   0
print_if_some:
        cmp     dword ptr [rdi], 0
        jne     .LBB0_2
        vmovdqu xmm0, xmmword ptr [rdi + 8]
        vpxor   xmm0, xmm0, xmmword ptr [rip + .LCPI0_0]
        vptest  xmm0, xmm0
        jne     .LBB0_2
        mov     edi, 42
        xor     esi, esi
        jmp     qword ptr [rip + do_not_elim@GOTPCREL]
.LBB0_2:
        ret
