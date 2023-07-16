
manually inlined (slow)              |LLVM-inlined (fast)
---------------------------------------------------------------------------
.LBB7_2:                             |.LBB7_2:
        cmpb    $1, (%r14)           |        cmpb    $1, (%r14)
        je      .LBB7_5              |        je      .LBB7_31
        movb    4(%r14), %al         |        movl    4(%r14), %eax
        andb    $31, %al             |        addl    $-45, %eax
        addb    $-13, %al            |        cmpl    $17, %eax
        movzbl  %al, %eax            |        ja      .LBB7_5
        cmpb    $17, %al             |        movslq  (%r15,%rax,4), %rax
        ja      .LBB7_8              |        addq    %r15, %rax
        movslq  (%r15,%rax,4), %rax  |        jmpq    *%rax
        addq    %r15, %rax           |.LBB7_7:
        jmpq    *%rax                | 
.LBB7_12:                            | 
