
b:
    pushq   %rax
    callq   *%rsi
    xorl    %eax, %eax
    popq    %rdx
    retq

c:
    pushq   %r14
    pushq   %rbx
    pushq   %rax
    movq    %rdi, %r14
    movq    (%r14), %rbx
    callq   *%rsi
    xorq    (%r14), %rbx
    movq    %rbx, %rax
    addq    $8, %rsp
    popq    %rbx
    popq    %r14
    retq
