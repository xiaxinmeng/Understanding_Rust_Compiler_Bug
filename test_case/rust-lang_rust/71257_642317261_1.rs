asm
foo(Vec&):                            # @foo(Vec&)
        pushq   %rax
        movq    (%rdi), %rax
        testq   %rax, %rax
        je      .LBB0_3
        movq    8(%rdi), %rcx
        movl    -4(%rcx,%rax,4), %eax
        cmpl    $2, %eax
        jge     .LBB0_4
        popq    %rcx
        retq
.LBB0_3:
        movl    $1, %edi
        callq   exit
.LBB0_4:
        movl    $3, %edi
        callq   exit
