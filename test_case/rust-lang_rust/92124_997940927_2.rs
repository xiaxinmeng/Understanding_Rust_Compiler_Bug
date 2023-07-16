asm
    .text
    .def     @feat.00;
    .scl    3;
    .type   0;
    .endef
    .globl  @feat.00
.set @feat.00, 0
    .file   "cstring_new.558cd7f7-cgu.0"
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
    subq    $32, %rsp
    .seh_stackalloc 32
    .seh_endprologue
    movq    %rcx, %rsi
    leaq    __unnamed_1(%rip), %rdx
    movl    $12, %r8d
    callq   _ZN66_$LT$$RF$str$u20$as$u20$std..ffi..c_str..CString..new..NewImpl$GT$8new_impl17ha5748a2c8ce696e6E
    movq    %rsi, %rax
    addq    $32, %rsp
    popq    %rsi
    retq
    .seh_endproc

    .section    .rdata,"dr",one_only,__unnamed_1
__unnamed_1:
    .ascii  "Hello world!"
