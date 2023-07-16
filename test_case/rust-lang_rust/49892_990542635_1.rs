assembly
example::cmp:
        cmp     dil, dl
        jne     .LBB0_1
        movzx   eax, dil
        lea     rdx, [rip + .LJTI0_0]
        movsxd  rax, dword ptr [rdx + 4*rax]
        add     rax, rdx
        jmp     rax
.LBB0_3:
        cmp     sil, cl
        sete    al
        ret
.LBB0_1:
        xor     eax, eax
        ret
.LJTI0_0:
        .long   .LBB0_3-.LJTI0_0
        .long   .LBB0_3-.LJTI0_0
        .long   .LBB0_3-.LJTI0_0
        .long   .LBB0_3-.LJTI0_0
        .long   .LBB0_3-.LJTI0_0
