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
    movq    16(%rcx), %r8
    cmpq    $2, %r8                ; Check length of input
    jb  .LBB0_14
    movq    (%rcx), %r10
    movl    (%r10), %eax
    leaq    -1(%r8), %r9
    xorl    %edx, %edx
    .p2align    4, 0x90
.LBB0_2:                           ; First loop header
    movl    %eax, %r11d
    movl    4(%r10,%rdx,4), %eax   ; Read item from vec to register
    cmpl    %eax, %r11d
    je  .LBB0_3                    ; Jump to code which handle item removes
    addq    $1, %rdx
    cmpq    %rdx, %r9
    jne .LBB0_2                    ; If we finished loop and didn't find duplicate, return
.LBB0_14:
    retq
.LBB0_3:
    leaq    2(%rdx), %r11
    leaq    1(%rdx), %r9
    cmpq    %r11, %r8
    jbe .LBB0_13
    movl    %r8d, %eax
    subl    %edx, %eax
    addl    $-2, %eax
    testb   $1, %al
    je  .LBB0_8
    movl    8(%r10,%rdx,4), %eax
    cmpl    (%r10,%rdx,4), %eax
    je  .LBB0_7
    movl    %eax, 4(%r10,%rdx,4)
    leaq    2(%rdx), %r9
.LBB0_7:
    leaq    3(%rdx), %r11
.LBB0_8:
    leaq    -3(%r8), %rax
    cmpq    %rdx, %rax
    jne .LBB0_9
.LBB0_13:
    movq    %r9, 16(%rcx)
    retq
    .p2align    4, 0x90
.LBB0_12:
    addq    $2, %r11
    cmpq    %r11, %r8
    je  .LBB0_13
.LBB0_9:
    movl    (%r10,%r11,4), %edx
    cmpl    -4(%r10,%r9,4), %edx
    jne .LBB0_16
    movl    4(%r10,%r11,4), %edx
    cmpl    -4(%r10,%r9,4), %edx
    je  .LBB0_12
    jmp .LBB0_11
    .p2align    4, 0x90
.LBB0_16:
    movl    %edx, (%r10,%r9,4)
    addq    $1, %r9
    movl    4(%r10,%r11,4), %edx
    cmpl    -4(%r10,%r9,4), %edx
    je  .LBB0_12
.LBB0_11:
    movl    %edx, (%r10,%r9,4)
    addq    $1, %r9
    jmp .LBB0_12
