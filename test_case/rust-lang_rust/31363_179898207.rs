 asm
.Ltmp806:
        .cfi_offset %rbp, -16
        testq   %rdx, %rdx
        je      .LBB2909_46
        leaq    -16(%rdx), %r11
        xorl    %ecx, %ecx
        leaq    _ZN3str15UTF8_CHAR_WIDTH20he77aff40c91d8da6qNSE(%rip), %r9
        movl    $49153, %r8d
        movabsq $-9187201950435737472, %r10
        .align  16, 0x90
.LBB2909_2:
        movzbl  (%rsi,%rcx), %r15d
        testb   %r15b, %r15b
        js      .LBB2909_3
