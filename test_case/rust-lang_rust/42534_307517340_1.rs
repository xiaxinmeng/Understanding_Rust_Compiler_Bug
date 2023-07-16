
_ZN5test29first_foo17hc5fe5ef1ed09d5a4E:
    testl   %ecx, %ecx
    jle .LBB0_1
    xorl    %edx, %edx
    xorl    %eax, %eax
    .p2align    4, 0x90
.LBB0_4:
    addl    %edx, %eax
    addl    $3, %edx
    cmpl    %ecx, %edx
    jl  .LBB0_4
    jmp .LBB0_2
.LBB0_1:
    xorl    %eax, %eax
.LBB0_2:
    retq


_ZN5test210second_foo17h5b0eb3168418bdd1E:
    movq    %rcx, %rdx
    shrq    $32, %rdx
    xorl    %eax, %eax
    cmpl    %edx, %ecx
    jge .LBB1_3
    xorl    %eax, %eax
    .p2align    4, 0x90
.LBB1_2:
    addl    %ecx, %eax
    addl    $3, %ecx
    cmovol  %edx, %ecx ; what is this for?
    cmpl    %edx, %ecx
    jl  .LBB1_2
.LBB1_3:
    retq


_ZN5test29third_foo17h179cbb320b2d4169E:
    movq    %rcx, %r9
    shrq    $32, %r9
    xorl    %r8d, %r8d
    xorl    %r10d, %r10d
    xorl    %eax, %eax
    testb   $1, %r10b
    je  .LBB2_11
    jmp .LBB2_2
    .p2align    4, 0x90
.LBB2_7:
    shrq    $32, %rdx
    addl    %eax, %edx
    movb    $1, %r10b
    movl    %edx, %eax
    testb   $1, %r10b
    je  .LBB2_11
.LBB2_2:
    cmpl    %r9d, %ecx
    jge .LBB2_5
    leal    1(%rcx), %edx
    cmpl    %r9d, %edx
    jge .LBB2_4
    leal    2(%rcx), %edx
    cmpl    %r9d, %edx
    jge .LBB2_9
    addl    $3, %ecx
    shlq    $32, %rdx
    movl    $1, %r10d
    jmp .LBB2_6
    .p2align    4, 0x90
.LBB2_11:
    movq    %rcx, %r10
    shlq    $32, %r10
    xorl    %edx, %edx
    cmpl    %r9d, %ecx
    setl    %dl
    cmovgeq %r8, %r10
    addl    %edx, %ecx
    jmp .LBB2_6
    .p2align    4, 0x90
.LBB2_4:
    movl    %edx, %ecx
    jmp .LBB2_5
.LBB2_9:
    movl    %edx, %ecx
    .p2align    4, 0x90
.LBB2_5:
    xorl    %edx, %edx
    xorl    %r10d, %r10d
.LBB2_6:
    orq %r10, %rdx
    testl   %edx, %edx
    jne .LBB2_7
    retq
