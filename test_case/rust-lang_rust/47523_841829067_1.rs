asm
example::bar:
        cmp     rsi, 3
        jne     .LBB0_4
        movzx   eax, byte ptr [rdi]
        add     eax, -48
        cmp     eax, 10
        jae     .LBB0_4
        movzx   eax, byte ptr [rdi + 1]
        add     eax, -48
        cmp     eax, 9
        ja      .LBB0_4
        movzx   eax, byte ptr [rdi + 2]
        add     eax, -48
        cmp     eax, 10
        setb    al
        ret
.LBB0_4:
        xor     eax, eax
        ret
