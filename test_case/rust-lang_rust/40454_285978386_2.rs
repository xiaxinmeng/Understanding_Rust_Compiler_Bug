as
# BB#0:
    xorl    %eax, %eax
    .p2align    4, 0x90
.LBB0_1:                                # =>This Inner Loop Header: Depth=1
    movups  (%rdi,%rax), %xmm0
    movups  16(%rdi,%rax), %xmm1
    movups  (%rsi,%rax), %xmm2
    movups  16(%rsi,%rax), %xmm3
    movups  %xmm2, (%rdi,%rax)
    movups  %xmm3, 16(%rdi,%rax)
    movups  %xmm0, (%rsi,%rax)
    movups  %xmm1, 16(%rsi,%rax)
    movups  32(%rdi,%rax), %xmm0
    movups  48(%rdi,%rax), %xmm1
    movups  32(%rsi,%rax), %xmm2
    movups  48(%rsi,%rax), %xmm3
    movups  %xmm2, 32(%rdi,%rax)
    movups  %xmm3, 48(%rdi,%rax)
    movups  %xmm0, 32(%rsi,%rax)
    movups  %xmm1, 48(%rsi,%rax)
    addq    $64, %rax
    cmpq    $2048, %rax             # imm = 0x800
    jne .LBB0_1
# BB#2:
    retq
