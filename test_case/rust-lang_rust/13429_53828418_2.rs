 gas
    .text
    .file   "test.ll"
    .globl  bar
    .align  16, 0x90
    .type   bar,@function
bar:                                    # @bar
    .cfi_startproc
# BB#0:                                 # %entry-block
    subl    $20, %esp
.Ltmp0:
    .cfi_def_cfa_offset 24
    movl    $1074339512, 12(%esp)   # imm = 0x40091EB8
    movl    $1374389535, 8(%esp)    # imm = 0x51EB851F 0x40091EB851EB851F (double) 3.14
    movl    $1078523331, 4(%esp)    # imm = 0x4048F5C3  0x4048F5C3 (float) 3.14
    addl    $20, %esp
    retl
.Ltmp1:
    .size   bar, .Ltmp1-bar
    .cfi_endproc


    .section    ".note.GNU-stack","",@progbits
