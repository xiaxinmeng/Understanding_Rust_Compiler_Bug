
$ eu-readelf -lS sample-edu-ciaa.elf
There are 23 section headers, starting at offset 0x1f9004:

Section Headers:
[Nr] Name                 Type         Addr     Off    Size   ES Flags Lk Inf Al
[ 0]                      NULL         00000000 000000 000000  0        0   0  0
[ 1] .stack               NOBITS       10000000 000114 001000  0 WA     0   0  1
[ 2] .text                PROGBITS     1a000000 000200 0112f8  0 AXM    0   0 16
[ 3] .ARM.exidx           ARM_EXIDX    1a0112f8 0114f8 000010  0 AL     2   0  4
[ 4] .storage             PROGBITS     1a011308 011508 0000f8  0 A      0   0  1
[ 5] .apps                PROGBITS     1a040000 011600 000000  0 A      0   0  1
[ 6] .relocate            PROGBITS     10001000 011600 0018a4  0 WA     0   0  4
[ 7] .sram                NOBITS       100028a4 012ea4 00575c  0 WA     0   0  4
[ 8] .debug_str           PROGBITS     00000000 012ea4 067400  1 MS     0   0  1
[ 9] .debug_loc           PROGBITS     00000000 07a2a4 043b4e  0        0   0  1
[10] .debug_abbrev        PROGBITS     00000000 0bddf2 001125  0        0   0  1
[11] .debug_info          PROGBITS     00000000 0bef17 09c23b  0        0   0  1
[12] .debug_aranges       PROGBITS     00000000 15b158 0017e8  0        0   0  8
[13] .debug_ranges        PROGBITS     00000000 15c940 00f678  0        0   0  1
[14] .debug_pubnames      PROGBITS     00000000 16bfb8 026ec5  0        0   0  1
[15] .debug_pubtypes      PROGBITS     00000000 192e7d 037bca  0        0   0  1
[16] .ARM.attributes      ARM_ATTRIBUTES 00000000 1caa47 000030  0        0   0  1
[17] .debug_frame         PROGBITS     00000000 1caa78 003018  0        0   0  4
[18] .debug_line          PROGBITS     00000000 1cda90 01da06  0        0   0  1
[19] .comment             PROGBITS     00000000 1eb496 000093  1 MS     0   0  1
[20] .symtab              SYMTAB       00000000 1eb52c 003ec0 16       22 972  4
[21] .shstrtab            STRTAB       00000000 1ef3ec 0000f1  0        0   0  1
[22] .strtab              STRTAB       00000000 1ef4dd 009b27  0        0   0  1

Program Headers:
  Type           Offset   VirtAddr   PhysAddr   FileSiz  MemSiz   Flg Align
  LOAD           0x000200 0x1a000000 0x1a000000 0x011400 0x011400 R E 0x200
  LOAD           0x011600 0x10001000 0x1a011400 0x0018a4 0x0018a4 RW  0x200
  GNU_STACK      0x000000 0x00000000 0x00000000 0x000000 0x000000 RW  0x0
  ARM_EXIDX      0x0114f8 0x1a0112f8 0x1a0112f8 0x000010 0x000010 R   0x4

 Section to Segment mapping:
  Segment Sections...
   00      [RO: .text .ARM.exidx .storage]
   01      .relocate
   02
   03      [RO: .ARM.exidx]
