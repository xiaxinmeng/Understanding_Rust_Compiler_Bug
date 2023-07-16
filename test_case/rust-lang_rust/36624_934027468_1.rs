asm
example::right_shift_1:
        mov     rax, rdi
        vmovdqa ymm0, ymmword ptr [rdx]
        vperm2i128      ymm1, ymm0, ymmword ptr [rsi], 3
        vpalignr        ymm0, ymm0, ymm1, 15
        vmovdqa ymmword ptr [rdi], ymm0
        vzeroupper
        ret
