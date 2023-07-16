asm
    .text
    .def     @feat.00;
    .scl    3;
    .type   0;
    .endef
    .globl  @feat.00
.set @feat.00, 0
    .file   "cstring_new.a85eb7bc-cgu.0"
    .def     make_c_string;
    .scl    2;
    .type   32;
    .endef
    .section    .text,"xr",one_only,make_c_string
    .globl  make_c_string
    .p2align    4, 0x90
make_c_string:
.seh_proc make_c_string
    pushq   %rsi
    .seh_pushreg %rsi
    pushq   %rdi
    .seh_pushreg %rdi
    subq    $56, %rsp
    .seh_stackalloc 56
    .seh_endprologue
    movq    %rcx, %rsi
    leaq    __unnamed_1(%rip), %rdx
    leaq    32(%rsp), %rdi
    movl    $12, %r8d
    movq    %rdi, %rcx
    callq   _ZN70_$LT$$RF$str$u20$as$u20$std..ffi..c_str..CString..new..SpecIntoVec$GT$8into_vec17h42e14556a4e783e0E
    movq    %rsi, %rcx
    movq    %rdi, %rdx
    callq   _ZN3std3ffi5c_str7CString4_new17hd06ee8640c50ad96E
    movq    %rsi, %rax
    addq    $56, %rsp
    popq    %rdi
    popq    %rsi
    retq
    .seh_endproc

    .section    .rdata,"dr",one_only,__unnamed_1
__unnamed_1:
    .ascii  "Hello world!"
