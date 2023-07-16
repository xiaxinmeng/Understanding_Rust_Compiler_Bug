asm
foo(Vec&):                            # @foo(Vec&)
        pushq   %rax
        movq    (%rdi), %rax
        testq   %rax, %rax
        je      .LBB0_2
        movq    8(%rdi), %rcx
        movl    -4(%rcx,%rax,4), %eax
        popq    %rcx
        retq
.LBB0_2:
        movl    $1, %edi
        callq   exit
