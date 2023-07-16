 asm
    .section    __TEXT,__text,regular,pure_instructions
    .section    __DWARF,__debug_info,regular,debug
Lsection_info:
    .section    __DWARF,__debug_abbrev,regular,debug
Lsection_abbrev:
    .section    __DWARF,__debug_line,regular,debug
Lsection_line:
    .section    __DWARF,__debug_str,regular,debug
Linfo_string:
    .section    __DWARF,__debug_loc,regular,debug
Lsection_debug_loc:
    .section    __DWARF,__debug_ranges,regular,debug
Ldebug_range:
    .section    __TEXT,__text,regular,pure_instructions
    .align  4, 0x90
__ZN4main20h8b95006aaacf56a1eaaE:
Lfunc_begin0:
    .file   1 "test2.rs"
    .loc    1 3 0
    .cfi_startproc
    cmpq    %gs:816, %rsp
    ja  LBB0_0
    movabsq $8, %r10
    movabsq $0, %r11
    callq   ___morestack
    retq
LBB0_0:
    pushq   %rbp
Ltmp0:
    .cfi_def_cfa_offset 16
Ltmp1:
    .cfi_offset %rbp, -16
    movq    %rsp, %rbp
Ltmp2:
    .cfi_def_cfa_register %rbp
    .loc    1 5 0 prologue_end
Ltmp3:
    ## InlineAsm Start
test:
    ## InlineAsm End
    popq    %rbp
    retq
Ltmp4:
Lfunc_end0:
    .cfi_endproc

    .globl  _main
    .align  4, 0x90
_main:
    .cfi_startproc
    cmpq    %gs:816, %rsp
    ja  LBB1_0
    movabsq $24, %r10
    movabsq $0, %r11
    callq   ___morestack
    retq
LBB1_0:
    pushq   %rbp
Ltmp5:
    .cfi_def_cfa_offset 16
Ltmp6:
    .cfi_offset %rbp, -16
    movq    %rsp, %rbp
Ltmp7:
    .cfi_def_cfa_register %rbp
    subq    $16, %rsp
    leaq    __ZN4main20h8b95006aaacf56a1eaaE(%rip), %rax
    movq    %rdi, -8(%rbp)
    movq    %rax, %rdi
    movq    -8(%rbp), %rax
    movq    %rsi, -16(%rbp)
    movq    %rax, %rsi
    movq    -16(%rbp), %rdx
    callq   __ZN10lang_start20h3eb2912460ad7fe8BueE
    addq    $16, %rsp
    popq    %rbp
    retq
    .cfi_endproc

Ldebug_end0:
    .section    __DWARF,__debug_str,regular,debug
Linfo_string0:
    .asciz  "rustc version 0.12.0-pre-nightly (a1429bca5 2014-08-08 21:36:11 +0000)"
Linfo_string1:
    .asciz  "./test2.rs"
Linfo_string2:
    .asciz  "/Users/enix/code/rusttest"
Linfo_string3:
    .asciz  "test2"
Linfo_string4:
    .asciz  "main"
Linfo_string5:
    .asciz  "_ZN5test24mainE"
    .section    __DWARF,__debug_info,regular,debug
L__DWARF__debug_info_begin0:
    .long   75
    .short  2
Lset0 = Lsection_abbrev-Lsection_abbrev
    .long   Lset0
    .byte   8
    .byte   1
Lset1 = Linfo_string0-Linfo_string
    .long   Lset1
    .short  36864
Lset2 = Linfo_string1-Linfo_string
    .long   Lset2
Lset3 = Lline_table_start0-Lsection_line
    .long   Lset3
Lset4 = Linfo_string2-Linfo_string
    .long   Lset4
    .quad   Lfunc_begin0
Lset5 = Lfunc_end0-Lfunc_begin0
    .long   Lset5
    .byte   2
Lset6 = Linfo_string3-Linfo_string
    .long   Lset6
    .byte   3
    .quad   Lfunc_begin0
    .quad   Lfunc_end0
    .byte   1
    .byte   86
Lset7 = Linfo_string5-Linfo_string
    .long   Lset7
Lset8 = Linfo_string4-Linfo_string
    .long   Lset8
    .byte   1
    .byte   3
    .byte   1
    .byte   0
    .byte   0
L__DWARF__debug_info_end0:
    .section    __DWARF,__debug_abbrev,regular,debug
    .byte   1
    .byte   17
    .byte   1
    .byte   37
    .byte   14
    .byte   19
    .byte   5
    .byte   3
    .byte   14
    .byte   16
    .byte   6
    .byte   27
    .byte   14
    .byte   17
    .byte   1
    .byte   18
    .byte   6
    .byte   0
    .byte   0
    .byte   2
    .byte   57
    .byte   1
    .byte   3
    .byte   14
    .byte   0
    .byte   0
    .byte   3
    .byte   46
    .byte   0
    .byte   17
    .byte   1
    .byte   18
    .byte   1
    .byte   64
    .byte   10
    .ascii  "\207@"
    .byte   14
    .byte   3
    .byte   14
    .byte   58
    .byte   11
    .byte   59
    .byte   11
    .byte   50
    .byte   11
    .byte   0
    .byte   0
    .byte   0
    .section    __DWARF,__debug_ranges,regular,debug
    .section    __DWARF,__debug_loc,regular,debug
    .section    __DWARF,__apple_names,regular,debug
Lnames_begin:
    .long   1212240712
    .short  1
    .short  0
    .long   2
    .long   2
    .long   12
    .long   0
    .long   1
    .short  1
    .short  6
    .long   0
    .long   1
    .long   2090499946
    .long   1217565201
    .long   LNames0-Lnames_begin
    .long   LNames1-Lnames_begin
LNames0:
Lset9 = Linfo_string4-Linfo_string
    .long   Lset9
    .long   1
    .long   47
    .long   0
LNames1:
Lset10 = Linfo_string5-Linfo_string
    .long   Lset10
    .long   1
    .long   47
    .long   0
    .section    __DWARF,__apple_objc,regular,debug
Lobjc_begin:
    .long   1212240712
    .short  1
    .short  0
    .long   1
    .long   0
    .long   12
    .long   0
    .long   1
    .short  1
    .short  6
    .long   -1
    .section    __DWARF,__apple_namespac,regular,debug
Lnamespac_begin:
    .long   1212240712
    .short  1
    .short  0
    .long   1
    .long   1
    .long   12
    .long   0
    .long   1
    .short  1
    .short  6
    .long   0
    .long   275477815
    .long   Lnamespac0-Lnamespac_begin
Lnamespac0:
Lset11 = Linfo_string3-Linfo_string
    .long   Lset11
    .long   1
    .long   42
    .long   0
    .section    __DWARF,__apple_types,regular,debug
Ltypes_begin:
    .long   1212240712
    .short  1
    .short  0
    .long   1
    .long   0
    .long   20
    .long   0
    .long   3
    .short  1
    .short  6
    .short  3
    .short  5
    .short  4
    .short  11
    .long   -1

.subsections_via_symbols
    .section    __DWARF,__debug_line,regular,debug
Lline_table_start0:
