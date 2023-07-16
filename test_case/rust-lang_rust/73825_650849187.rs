asm
.LCPI0_0:
        .long   1
        .long   3
        .long   4
        .long   5
example::foo:
        sub     rsp, 24
        movaps  xmm0, xmmword ptr [rip + .LCPI0_0]
        movaps  xmmword ptr [rsp], xmm0
        mov     dword ptr [rsp + 16], 2
        movabs  rcx, -3689348814741910323
        mov     rax, rdi
        mul     rcx
        shr     rdx, 2
        lea     rax, [rdx + 4*rdx]
        sub     rdi, rax
        mov     eax, dword ptr [rsp + 4*rdi]
        add     rsp, 24
        ret
