 asm
    subq    $24, %rsp
.Ltmp4:
    .cfi_def_cfa_offset 32
    movaps  32(%rsp), %xmm0
    movups  %xmm0, (%rsp)
    callq   _ZN3bar20h6b5879ee29f32fdeeaaE@PLT
    movaps  32(%rsp), %xmm0
    movups  %xmm0, (%rsp)
    callq   _ZN3bar20h6b5879ee29f32fdeeaaE@PLT
    addq    $24, %rsp
    retq
