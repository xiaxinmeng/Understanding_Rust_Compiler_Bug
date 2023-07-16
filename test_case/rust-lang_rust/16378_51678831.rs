 asm
    .section    __TEXT,__text,regular,pure_instructions
    .align  4, 0x90
__ZN4main20h8b95006aaacf56a1eaaE:
    .cfi_startproc
    cmpq    %gs:816, %rsp
    ja  LBB0_2
    movabsq $8, %r10
    movabsq $0, %r11
    callq   ___morestack
    retq
LBB0_2:
    pushq   %rbp
Ltmp0:
    .cfi_def_cfa_offset 16
Ltmp1:
    .cfi_offset %rbp, -16
    movq    %rsp, %rbp
Ltmp2:
    .cfi_def_cfa_register %rbp
    ## InlineAsm Start
test:
    ## InlineAsm End
    popq    %rbp
    retq
    .cfi_endproc

    .globl  _main
    .align  4, 0x90
_main:
    .cfi_startproc
    cmpq    %gs:816, %rsp
    ja  LBB1_2
    movabsq $8, %r10
    movabsq $0, %r11
    callq   ___morestack
    retq
LBB1_2:
    pushq   %rbp
Ltmp3:
    .cfi_def_cfa_offset 16
Ltmp4:
    .cfi_offset %rbp, -16
    movq    %rsp, %rbp
Ltmp5:
    .cfi_def_cfa_register %rbp
    movq    %rsi, %rax
    movq    %rdi, %rcx
    leaq    __ZN4main20h8b95006aaacf56a1eaaE(%rip), %rdi
    movq    %rcx, %rsi
    movq    %rax, %rdx
    popq    %rbp
    jmp __ZN10lang_start20h3eb2912460ad7fe8BueE
    .cfi_endproc


.subsections_via_symbols
