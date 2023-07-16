as
swap_array_u64x4:
    .cfi_startproc
    xorl    %eax, %eax
    .p2align    4, 0x90
.LBB0_1:
    movups  (%rdi,%rax), %xmm0
    movups  16(%rdi,%rax), %xmm1
    movups  (%rsi,%rax), %xmm2
    movups  16(%rsi,%rax), %xmm3
    movups  %xmm3, 16(%rdi,%rax)
    movups  %xmm2, (%rdi,%rax)
    movups  %xmm1, 16(%rsi,%rax)
    movups  %xmm0, (%rsi,%rax)
    movups  32(%rdi,%rax), %xmm0
    movups  48(%rdi,%rax), %xmm1
    movups  32(%rsi,%rax), %xmm2
    movups  48(%rsi,%rax), %xmm3
    movups  %xmm3, 48(%rdi,%rax)
    movups  %xmm2, 32(%rdi,%rax)
    movups  %xmm1, 48(%rsi,%rax)
    movups  %xmm0, 32(%rsi,%rax)
    movups  64(%rdi,%rax), %xmm0
    movups  80(%rdi,%rax), %xmm1
    movups  64(%rsi,%rax), %xmm2
    movups  80(%rsi,%rax), %xmm3
    movups  %xmm3, 80(%rdi,%rax)
    movups  %xmm2, 64(%rdi,%rax)
    movups  %xmm1, 80(%rsi,%rax)
    movups  %xmm0, 64(%rsi,%rax)
    movups  96(%rdi,%rax), %xmm0
    movups  112(%rdi,%rax), %xmm1
    movups  96(%rsi,%rax), %xmm2
    movups  112(%rsi,%rax), %xmm3
    movups  %xmm3, 112(%rdi,%rax)
    movups  %xmm2, 96(%rdi,%rax)
    movups  %xmm1, 112(%rsi,%rax)
    movups  %xmm0, 96(%rsi,%rax)
    movq    %rax, %rcx
    subq    $-128, %rcx
    addq    $160, %rax
    cmpq    $2049, %rax
    movq    %rcx, %rax
    jl  .LBB0_1
    retq
