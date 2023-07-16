 asm
    .type   _ZN8get_bool20hd181be98fe457b77eaaE,@function
_ZN8get_bool20hd181be98fe457b77eaaE:
    .cfi_startproc
    movzbl  (%rdi), %eax
    cmpl    $1, %eax
    je  .LBB0_2
    movzbl  %sil, %eax
    shll    $8, %eax
    orl $1, %eax
    movw    %ax, (%rdi)
.LBB0_2:
    incq    %rdi
    movq    %rdi, %rax
    retq
.Ltmp0:
    .size   _ZN8get_bool20hd181be98fe457b77eaaE, .Ltmp0-_ZN8get_bool20hd181be98fe457b77eaaE
