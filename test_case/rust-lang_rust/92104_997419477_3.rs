asm
    .text
    .def     @feat.00;
    .scl    3;
    .type   0;
    .endef
    .globl  @feat.00
.set @feat.00, 0
    .file   "dedup.f421260b-cgu.0"
    .def     _ZN5dedup5dedup17h0bbecfdc6851a7d9E;
    .scl    2;
    .type   32;
    .endef
    .section    .text,"xr",one_only,_ZN5dedup5dedup17h0bbecfdc6851a7d9E
    .globl  _ZN5dedup5dedup17h0bbecfdc6851a7d9E
    .p2align    4, 0x90
_ZN5dedup5dedup17h0bbecfdc6851a7d9E:
    movq    16(%rcx), %rax
    cmpq    $2, %rax
    jb  .LBB0_7
    movq    (%rcx), %r9
    leaq    -1(%rax), %r8
    cmpq    $2, %rax
    jne .LBB0_8
    movl    $1, %eax
    movl    $1, %r11d
.LBB0_3:
    testb   $1, %r8b
    je  .LBB0_6
    movl    (%r9,%rax,4), %eax
    cmpl    -4(%r9,%r11,4), %eax
    je  .LBB0_6
    movl    %eax, (%r9,%r11,4)
    addq    $1, %r11
.LBB0_6:
    movq    %r11, 16(%rcx)
.LBB0_7:
    retq
.LBB0_8:
    movq    %r8, %r10
    andq    $-2, %r10
    negq    %r10
    movl    $1, %eax
    movl    $1, %r11d
    jmp .LBB0_9
    .p2align    4, 0x90
.LBB0_12:
    leaq    (%r10,%rax), %rdx
    addq    $2, %rdx
    addq    $2, %rax
    cmpq    $1, %rdx
    je  .LBB0_3
.LBB0_9:
    movl    (%r9,%rax,4), %edx
    cmpl    -4(%r9,%r11,4), %edx
    je  .LBB0_10
    movl    %edx, (%r9,%r11,4)
    addq    $1, %r11
.LBB0_10:
    movl    4(%r9,%rax,4), %edx
    cmpl    -4(%r9,%r11,4), %edx
    je  .LBB0_12
    movl    %edx, (%r9,%r11,4)
    addq    $1, %r11
    jmp .LBB0_12
