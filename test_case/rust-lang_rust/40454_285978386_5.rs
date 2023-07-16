as
swap_array:
    .cfi_startproc
    xorl    %eax, %eax
    .p2align    4, 0x90
.LBB0_1:
    movups  (%rdi,%rax), %xmm0
    movaps  %xmm0, -24(%rsp)
    movups  (%rsi,%rax), %xmm0
    movups  %xmm0, (%rdi,%rax)
    movaps  -24(%rsp), %xmm0
    movups  %xmm0, (%rsi,%rax)
    movups  16(%rdi,%rax), %xmm0
    movaps  %xmm0, -24(%rsp)
    movups  16(%rsi,%rax), %xmm0
    movups  %xmm0, 16(%rdi,%rax)
    movaps  -24(%rsp), %xmm0
    movups  %xmm0, 16(%rsi,%rax)
    movups  32(%rdi,%rax), %xmm0
    movaps  %xmm0, -24(%rsp)
    movups  32(%rsi,%rax), %xmm0
    movups  %xmm0, 32(%rdi,%rax)
    movaps  -24(%rsp), %xmm0
    movups  %xmm0, 32(%rsi,%rax)
    movups  48(%rdi,%rax), %xmm0
    movaps  %xmm0, -24(%rsp)
    movups  48(%rsi,%rax), %xmm0
    movups  %xmm0, 48(%rdi,%rax)
    movaps  -24(%rsp), %xmm0
    movups  %xmm0, 48(%rsi,%rax)
    leaq    64(%rax), %rcx
    addq    $80, %rax
    cmpq    $2049, %rax
    movq    %rcx, %rax
    jl  .LBB0_1
    retq
