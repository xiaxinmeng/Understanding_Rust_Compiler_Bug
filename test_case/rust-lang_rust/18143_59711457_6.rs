 asm
.LBB0_2:                                # %._crit_edge
                                        #   in Loop: Header=BB0_1 Depth=1
    decq    %rbx
.LBB0_1:                                # %.outer
                                        # =>This Inner Loop Header: Depth=1
    cmpb    $62, (%rbx)
    jne .LBB0_2
# BB#3:                                 # %.lr.ph
                                        #   in Loop: Header=BB0_1 Depth=1
    movq    %rbx, %rdi
    callq   black
    jmp .LBB0_1
