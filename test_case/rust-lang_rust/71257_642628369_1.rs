asm
foo(Foo*):                            # @foo(Foo*)
        pushq   %rax
        movl    4(%rdi), %eax
        cmpl    $3, %eax
        je      .LBB0_2
        popq    %rcx
        retq
.LBB0_2:
        movl    $3, %edi
        callq   exit
