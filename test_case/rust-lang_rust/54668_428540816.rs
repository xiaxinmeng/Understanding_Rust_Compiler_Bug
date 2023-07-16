asm
        vmovups ymm0, ymmword ptr [rbx + rax]
        vmovaps ymmword ptr [rsp], ymm0
        vmovups ymm0, ymmword ptr [r15 + rax]
        vmovups ymmword ptr [rbx + rax], ymm0
        vmovaps ymm0, ymmword ptr [rsp]
        vmovups ymmword ptr [r15 + rax], ymm0
