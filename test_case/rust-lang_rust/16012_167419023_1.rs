 asm
...
.Ltmp8:
    subq    $999999888, %rsp        ; subtract 999999888 from the stack pointer
    xorl    %eax, %eax
    movl    %eax, %ecx
    leaq    -1000000000(%rbp), %rdx
    movb    $61, -1(%rbp)
    movq    %rdx, -1000000008(%rbp)
    movq    %rcx, -1000000016(%rbp)
.LBB1_1:
    movq    -1000000016(%rbp), %rax
    movq    -1000000008(%rbp), %rcx
    movb    $0, (%rcx,%rax)         ; write a 0 byte to memory at (rcx, rax)
    addq    $1, %rax                ; increase rax by 1
    cmpq    $999999999, %rax
    movq    %rax, -1000000016(%rbp)
    jb  .LBB1_1
    .loc    1 9 0 prologue_end
...
