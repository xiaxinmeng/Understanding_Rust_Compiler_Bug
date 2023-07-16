 asm
    .section    __TEXT,__text,regular,pure_instructions
    .align  4, 0x90
__ZN10light_fuse4anon7expr_fn2anE:      ## @_ZN10light_fuse4anon7expr_fn2anE
    .cfi_startproc
## BB#0:                                ## %function top level
    subq    $56, %rsp
Ltmp12:
    .cfi_def_cfa_offset 64
    movabsq $0, %rax
    leaq    40(%rsp), %rsi
    movq    32(%rdi), %rdi
    movq    (%rdi), %rcx
    movq    %rcx, 40(%rsp)
    movq    8(%rdi), %rcx
    movq    %rcx, 48(%rsp)
    movq    %rax, %rdi
    callq   __ZN20_$SP$bomb.$x27static9glue_take19h9f15e2dd62f678e4arE
    leaq    32(%rsp), %rdi
                                        ## implicit-def: RSI
    movq    48(%rsp), %rax
    movq    40(%rsp), %rcx
    movq    8(%rcx), %rcx
    movq    %rcx, 8(%rsp)           ## 8-byte Spill
    movq    %rax, (%rsp)            ## 8-byte Spill
    callq   __ZN9Ident_new16h4688b2c57373e544v0.0E
    leaq    32(%rsp), %rsi
    movq    (%rsp), %rdi            ## 8-byte Reload
    movq    8(%rsp), %rax           ## 8-byte Reload
    callq   *%rax
