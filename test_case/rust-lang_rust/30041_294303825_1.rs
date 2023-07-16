asm
bad1(bool*):                              # @bad1(bool*)
        cmpb    $0, (%rdi)
        je      .LBB0_2
        movb    $1, (%rdi)
.LBB0_2:
        retq

bad2(bool*):                              # @bad2(bool*)
        cmpb    $0, (%rdi)
        je      .LBB1_2
        movb    $1, (%rdi)
.LBB1_2:
        retq

bad3(bool*):                              # @bad3(bool*)
        cmpb    $0, (%rdi)
        jne     .LBB2_2
        movb    $0, (%rdi)
.LBB2_2:
        retq
