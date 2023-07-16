 asm
_ZN5other20h3127dd56c08a3ac5baaE:
    .cfi_startproc
    jmp .LBB0_1
.LBB0_1:
    movl    $65535, %eax
    retl

...

_ZN9ext_other20ha9d431dfe83be901jaaE:
    .cfi_startproc
    pushl   %ebx
.Ltmp1:
    .cfi_def_cfa_offset 8
    subl    $8, %esp
.Ltmp2:
    .cfi_def_cfa_offset 16
.Ltmp3:
    .cfi_offset %ebx, -8
    calll   .L1$pb
.L1$pb:
    popl    %eax
.Ltmp4:
    addl    $_GLOBAL_OFFSET_TABLE_+(.Ltmp4-.L1$pb), %eax
    movl    %eax, %ebx
    calll   _ZN9ext_other10__rust_abiE
    addl    $8, %esp
    popl    %ebx
    retl

...

_ZN9ext_other10__rust_abiE:
    .cfi_startproc
    jmp .LBB2_1
.LBB2_1:
    movl    $43690, %eax
    retl
