 asm
    .section    __TEXT,__text,regular,pure_instructions
    .align  4, 0x90
__ZN10light_fuse4anon7expr_fn2anE:      ## @_ZN10light_fuse4anon7expr_fn2anE
    .cfi_startproc
    .cfi_personality 155, _upcall_rust_personality
Leh_func_begin5:
    .cfi_lsda 16, Lexception5
## BB#0:                                ## %function top level
    subq    $88, %rsp
Ltmp15:
    .cfi_def_cfa_offset 96
    movq    32(%rdi), %rdi
    vmovups (%rdi), %xmm0
    vmovaps %xmm0, 64(%rsp)
    xorl    %eax, %eax
    movl    %eax, %edi
    leaq    64(%rsp), %rsi
    callq   __ZN20_$SP$bomb.$x27static9glue_take19h9f15e2dd62f678e4arE
    movq    64(%rsp), %rsi
    movq    72(%rsp), %rdi
    movq    8(%rsi), %rsi
Ltmp11:
    leaq    56(%rsp), %rcx
    movq    %rdi, 32(%rsp)          ## 8-byte Spill
    movq    %rcx, %rdi
                                        ## implicit-def: RCX
    movq    %rsi, 24(%rsp)          ## 8-byte Spill
    movq    %rcx, %rsi
    callq   __ZN9Ident_new16h4688b2c57373e544v0.0E
Ltmp12:
    jmp LBB5_1
LBB5_1:                                 ## %normal return
    leaq    56(%rsp), %rsi
    movq    32(%rsp), %rdi          ## 8-byte Reload
    movq    16(%rsp), %rax          ## 8-byte Reload
    callq   *%rax
