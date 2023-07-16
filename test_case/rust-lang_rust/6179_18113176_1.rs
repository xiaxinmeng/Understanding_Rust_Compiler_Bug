
_ZN3foo17_1773c175964f18a23_00E:
    .cfi_startproc
    cmpl    %gs:48, %esp
    ja  .LBB3_2
    pushl   $16
    pushl   $4
    calll   __morestack
    ret
.LBB3_2:
    pushl   %ebp
.Ltmp15:
    .cfi_def_cfa_offset 8
.Ltmp16:
    .cfi_offset %ebp, -8
    movl    %esp, %ebp
.Ltmp17:
    .cfi_def_cfa_register %ebp
    popl    %ebp
    ret
.Ltmp18:
    .size   _ZN3foo17_1773c175964f18a23_00E, .Ltmp18-_ZN3foo17_1773c175964f18a23_00E
    .cfi_endproc
