 asm
    .text
    .file   "issue14912.0.rs"
    .section    .text.main,"ax",@progbits
    .globl  main
    .align  16, 0x90
    .type   main,@function
main:
    xorl    %eax, %eax
    retq
.Ltmp0:
    .size   main, .Ltmp0-main


    .section    ".note.GNU-stack","",@progbits
