
$ clang -v
clang version 3.8.0-2ubuntu4 (tags/RELEASE_380/final)

$ cat foo.c
int main() {
  while (1) {}
}

$ clang --target=msp430 foo.c

$ msp430-readelf -h a.out
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 ff 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            Standalone App
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           Texas Instruments msp430 microcontroller
  Version:                           0x1
  Entry point address:               0xf800
  Start of program headers:          52 (bytes into file)
  Start of section headers:          2576 (bytes into file)
  Flags:                             0x10000000
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         3
  Size of section headers:           40 (bytes)
  Number of section headers:         12
  Section header string table index: 9

$ msp430-objdump -Cd a.out

a.out:     file format elf32-msp430


Disassembly of section .text:

0000f800 <__watchdog_support>:
    f800:       55 42 20 01     mov.b   &0x0120,r5
    f804:       35 d0 08 5a     bis     #23048, r5      ;#0x5a08
    f808:       82 45 00 02     mov     r5,     &0x0200

0000f80c <__init_stack>:
    f80c:       31 40 00 03     mov     #768,   r1      ;#0x0300

0000f810 <__do_copy_data>:
    f810:       3f 40 00 00     mov     #0,     r15     ;#0x0000
    f814:       0f 93           tst     r15
    f816:       08 24           jz      $+18            ;abs 0xf828
    f818:       92 42 00 02     mov     &0x0200,&0x0120
    f81c:       20 01
    f81e:       2f 83           decd    r15
    f820:       9f 4f 5a f8     mov     -1958(r15),512(r15);0xf85a(r15), 0x0200(r15)
    f824:       00 02
    f826:       f8 23           jnz     $-14            ;abs 0xf818

0000f828 <__do_clear_bss>:
    f828:       3f 40 00 00     mov     #0,     r15     ;#0x0000
    f82c:       0f 93           tst     r15
    f82e:       07 24           jz      $+16            ;abs 0xf83e
    f830:       92 42 00 02     mov     &0x0200,&0x0120
    f834:       20 01
    f836:       1f 83           dec     r15
    f838:       cf 43 00 02     mov.b   #0,     512(r15);r3 As==00, 0x0200(r15)
    f83c:       f9 23           jnz     $-12            ;abs 0xf830

0000f83e <__stop_progExec__>:
    f83e:       32 d0 f0 00     bis     #240,   r2      ;#0x00f0
    f842:       fd 3f           jmp     $-4             ;abs 0xf83e

0000f844 <__ctors_end>:
    f844:       30 40 58 f8     br      #0xf858

0000f848 <main>:
    f848:       04 12           push    r4
    f84a:       04 41           mov     r1,     r4
    f84c:       21 83           decd    r1
    f84e:       84 43 fe ff     mov     #0,     -2(r4)  ;r3 As==00, 0xfffe(r4)
    f852:       00 3c           jmp     $+2             ;abs 0xf854
    f854:       ff 3f           jmp     $+0             ;abs 0xf854
        ...

0000f858 <_unexpected_>:
    f858:       00 13           reti

Disassembly of section .vectors:

0000ffe0 <__ivtbl_16>:
    ffe0:       44 f8 44 f8 44 f8 44 f8 44 f8 44 f8 44 f8 44 f8     D.D.D.D.D.D.D.D.
    fff0:       44 f8 44 f8 44 f8 44 f8 44 f8 44 f8 44 f8 00 f8     D.D.D.D.D.D.D...
