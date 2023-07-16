 asm
    .type   _ZN14__extensions__9meth_28651m17_7e6e9d6cd229cfd23_00E,@function
_ZN14__extensions__9meth_28651m17_7e6e9d6cd229cfd23_00E:
    .cfi_startproc
    cmpq    %fs:112, %rsp
    ja  .LBB0_0
    movabsq $40, %r10
    movabsq $0, %r11
    callq   __morestack
    ret
.LBB0_0:
    pushq   %rbp
.Ltmp2:
    .cfi_def_cfa_offset 16
.Ltmp3:
    .cfi_offset %rbp, -16
    movq    %rsp, %rbp
.Ltmp4:
    .cfi_def_cfa_register %rbp
    subq    $32, %rsp
    movq    %rdi, -16(%rbp)
    jmp .LBB0_2
.LBB0_1:
    addq    $32, %rsp
    popq    %rbp
    ret
.LBB0_2:
    movq    -16(%rbp), %rax
    movq    %rax, -24(%rbp)
    movabsq $0, %rax
    movq    %rax, %rdi
    movq    %rax, %rsi
    movq    -24(%rbp), %rdx
    callq   _ZN9_ubox_u3217_d8e692bd4656d07e14glue_drop_2877E
    jmp .LBB0_1
.Ltmp5:
    .size   _ZN14__extensions__9meth_28651m17_7e6e9d6cd229cfd23_00E, .Ltmp5-_ZN14__extensions__9meth_28651m17_7e6e9d6cd229cfd23_00E

