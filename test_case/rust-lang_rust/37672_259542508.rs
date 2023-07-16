
$ clang --target=msp430 -c foo.c -o foo.o

$ file foo.o
foo.o: ELF 32-bit LSB executable, TI msp430, version 1, statically linked, stripped

$ msp430-objdump -Cd foo.o

foo.o:     file format elf32-msp430


Disassembly of section .text:

0000f800 <.text>:
    f800:       55 42 20 01     mov.b   &0x0120,r5
    f804:       35 d0 08 5a     bis     #23048, r5      ;#0x5a08
    f808:       82 45 00 02     mov     r5,     &0x0200
    f80c:       31 40 00 03     mov     #768,   r1      ;#0x0300
    f810:       3f 40 00 00     mov     #0,     r15     ;#0x0000
    f814:       0f 93           tst     r15
    f816:       08 24           jz      $+18            ;abs 0xf828
    f818:       92 42 00 02     mov     &0x0200,&0x0120
    f81c:       20 01
    f81e:       2f 83           decd    r15
    f820:       9f 4f 5a f8     mov     -1958(r15),512(r15);0xf85a(r15), 0x0200(r15)
    f824:       00 02
    f826:       f8 23           jnz     $-14            ;abs 0xf818
    f828:       3f 40 00 00     mov     #0,     r15     ;#0x0000
    f82c:       0f 93           tst     r15
    f82e:       07 24           jz      $+16            ;abs 0xf83e
    f830:       92 42 00 02     mov     &0x0200,&0x0120
    f834:       20 01
    f836:       1f 83           dec     r15
    f838:       cf 43 00 02     mov.b   #0,     512(r15);r3 As==00, 0x0200(r15)
    f83c:       f9 23           jnz     $-12            ;abs 0xf830
    f83e:       32 d0 f0 00     bis     #240,   r2      ;#0x00f0
    f842:       fd 3f           jmp     $-4             ;abs 0xf83e
    f844:       30 40 58 f8     br      #0xf858
    f848:       04 12           push    r4
    f84a:       04 41           mov     r1,     r4
    f84c:       21 83           decd    r1
    f84e:       84 43 fe ff     mov     #0,     -2(r4)  ;r3 As==00, 0xfffe(r4)
    f852:       00 3c           jmp     $+2             ;abs 0xf854
    f854:       ff 3f           jmp     $+0             ;abs 0xf854
    f856:       00 00           .word   0x0000; ????
    f858:       00 13           reti

Disassembly of section .vectors:

0000ffe0 <.vectors>:
    ffe0:       44 f8           and.b   r8,     r4
    ffe2:       44 f8           and.b   r8,     r4
    ffe4:       44 f8           and.b   r8,     r4
    ffe6:       44 f8           and.b   r8,     r4
    ffe8:       44 f8           and.b   r8,     r4
    ffea:       44 f8           and.b   r8,     r4
    ffec:       44 f8           and.b   r8,     r4
    ffee:       44 f8           and.b   r8,     r4
    fff0:       44 f8           and.b   r8,     r4
    fff2:       44 f8           and.b   r8,     r4
    fff4:       44 f8           and.b   r8,     r4
    fff6:       44 f8           and.b   r8,     r4
    fff8:       44 f8           and.b   r8,     r4
    fffa:       44 f8           and.b   r8,     r4
    fffc:       44 f8           and.b   r8,     r4
    fffe:       00 f8           and     r8,     r0
