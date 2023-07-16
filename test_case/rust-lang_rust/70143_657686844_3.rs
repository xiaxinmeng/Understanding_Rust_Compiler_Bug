asm
example::foo:
        pushq   %rbp
        movq    %rsp, %rbp
        andq    $-268435456, %rsp    # 0xfffffffff0000000
        movq    %rsp, %r11
        subq    $536870912, %r11    # 0x20000000
.LBB0_1:
        subq    $4096, %rsp
        movq    $0, (%rsp)
        cmpq    %r11, %rsp
        jne     .LBB0_1
        movq    $2, (%rsp)
        movq    (%rsp), %rax
        movq    %rbp, %rsp
        popq    %rbp
        retq
