asm
        vpcmpeqb        k1, ymm1, ymmword ptr [r11 + rax - 224]
        vpcmpeqb        k2, ymm1, ymmword ptr [r11 + rax - 192]
        vpcmpeqb        k3, ymm1, ymmword ptr [r11 + rax - 160]
        vpcmpeqb        k4, ymm1, ymmword ptr [r11 + rax - 128]
        vmovdqu8        ymmword ptr [r11 + rax - 224] {k1}, ymm0
        vmovdqu8        ymmword ptr [r11 + rax - 192] {k2}, ymm0
        vmovdqu8        ymmword ptr [r11 + rax - 160] {k3}, ymm0
        vmovdqu8        ymmword ptr [r11 + rax - 128] {k4}, ymm0
        vpcmpeqb        k1, ymm1, ymmword ptr [r11 + rax - 96]
        vpcmpeqb        k2, ymm1, ymmword ptr [r11 + rax - 64]
        vpcmpeqb        k3, ymm1, ymmword ptr [r11 + rax - 32]
        vpcmpeqb        k4, ymm1, ymmword ptr [r11 + rax]
        vmovdqu8        ymmword ptr [r11 + rax - 96] {k1}, ymm0
        vmovdqu8        ymmword ptr [r11 + rax - 64] {k2}, ymm0
        vmovdqu8        ymmword ptr [r11 + rax - 32] {k3}, ymm0
        vmovdqu8        ymmword ptr [r11 + rax] {k4}, ymm0
