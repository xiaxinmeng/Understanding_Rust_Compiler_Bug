
    subl    $36, %esp
.Ltmp12:
    .cfi_offset %ebx, -12
    calll   .L1$pb
.L1$pb:
    popl    %eax
.Ltmp13:
    addl    $_GLOBAL_OFFSET_TABLE_+(.Ltmp13-.L1$pb), %eax
    leal    -16(%ebp), %ecx
    movl    %ecx, (%esp)
    movl    %eax, %ebx
    movl    %eax, -28(%ebp)
    calll   rust_dbg_extern_return_TwoU32s@PLT
    subl    $4, %esp // XXX
    leal    -24(%ebp), %eax
    movl    %eax, (%esp)
    movl    -28(%ebp), %ebx
    calll   rust_dbg_extern_return_TwoU32s@PLT
    addl    $36, %esp
    popl    %ebx
    popl    %ebp
    ret
