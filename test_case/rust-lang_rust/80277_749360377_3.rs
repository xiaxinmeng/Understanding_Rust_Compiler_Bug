
        callq   _ZN66_$LT$std..io..cursor..Cursor$LT$T$GT$$u20$as$u20$std..io..Read$GT$10read_exact17hc00b0cae91b17a7bE
        cmpb    $3, %al
        jne     .LBB12_5
        movzbl  1(%rsp), %r12d
        movb    $0, 1(%rsp)
        leaq    1(%rsp), %rsi
        movl    $1, %r15d
        movl    $1, %edx
        movq    %r14, %rdi
        callq   _ZN66_$LT$std..io..cursor..Cursor$LT$T$GT$$u20$as$u20$std..io..Read$GT$10read_exact17hc00b0cae91b17a7bE
        cmpb    $3, %al
        jne     .LBB12_5
        movzbl  1(%rsp), %r13d
        movw    $0, 2(%rsp)
        leaq    2(%rsp), %rsi
        movl    $2, %edx
        movq    %r14, %rdi
        callq   _ZN66_$LT$std..io..cursor..Cursor$LT$T$GT$$u20$as$u20$std..io..Read$GT$10read_exact17hc00b0cae91b17a7bE
        cmpb    $3, %al
        jne     .LBB12_5
        movzwl  2(%rsp), %ebp
        movl    $0, 4(%rsp)
        leaq    4(%rsp), %rsi
        movl    $4, %edx
        movq    %r14, %rdi
        callq   _ZN66_$LT$std..io..cursor..Cursor$LT$T$GT$$u20$as$u20$std..io..Read$GT$10read_exact17hc00b0cae91b17a7bE
        cmpb    $3, %al
        jne     .LBB12_5
        addq    %r12, %r13
        addq    %r13, %rbp
        movl    4(%rsp), %eax
        addq    %rbp, %rax
        movq    %rax, 8(%rbx)
        xorl    %r15d, %r15d
        jmp     .LBB12_6
