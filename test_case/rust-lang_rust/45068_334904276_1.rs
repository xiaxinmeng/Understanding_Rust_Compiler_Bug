assembly
        movups  (%rsi), %xmm0
        movups  %xmm0, (%rax)
        movups  16(%rsi), %xmm0
        movups  %xmm0, 16(%rax)
        movl    32(%rsi), %ecx
        movl    %ecx, 32(%rax)
        movl    36(%rsi), %ecx
        movl    %ecx, 36(%rax)
        movl    40(%rsi), %ecx
        movl    %ecx, 40(%rax)
