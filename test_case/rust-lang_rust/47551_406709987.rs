
$ readelf -S ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/crti.o 
There are 19 section headers, starting at offset 0x4a0:

Section Headers:
  [Nr] Name              Type             Address           Offset
       Size              EntSize          Flags  Link  Info  Align
  [ 0]                   NULL             0000000000000000  00000000
       0000000000000000  0000000000000000           0     0     0
  [ 1] .text             PROGBITS         0000000000000000  00000040
       0000000000000000  0000000000000000  AX       0     0     1
  [ 2] .data             PROGBITS         0000000000000000  00000040
       0000000000000000  0000000000000000  WA       0     0     1
  [ 3] .bss              NOBITS           0000000000000000  00000040
       0000000000000000  0000000000000000  WA       0     0     1
  [ 4] .init             PROGBITS         0000000000000000  00000040
       0000000000000001  0000000000000000  AX       0     0     1
  [ 5] .fini             PROGBITS         0000000000000000  00000041
       0000000000000001  0000000000000000  AX       0     0     1
  [ 6] .note.GNU-stack   PROGBITS         0000000000000000  00000042
       0000000000000000  0000000000000000           0     0     1
  [ 7] .debug_line       PROGBITS         0000000000000000  00000042
       0000000000000056  0000000000000000           0     0     1
  [ 8] .rela.debug_line  RELA             0000000000000000  000002e0
       0000000000000030  0000000000000018   I      17     7     8
  [ 9] .debug_info       PROGBITS         0000000000000000  00000098
       0000000000000049  0000000000000000           0     0     1
  [10] .rela.debug_info  RELA             0000000000000000  00000310
       0000000000000048  0000000000000018   I      17     9     8
  [11] .debug_abbrev     PROGBITS         0000000000000000  000000e1
       0000000000000012  0000000000000000           0     0     1
  [12] .debug_aranges    PROGBITS         0000000000000000  00000100
       0000000000000040  0000000000000000           0     0     16
  [13] .rela.debug_arang RELA             0000000000000000  00000358
       0000000000000048  0000000000000018   I      17    12     8
  [14] .debug_ranges     PROGBITS         0000000000000000  00000140
       0000000000000040  0000000000000000           0     0     16
  [15] .rela.debug_range RELA             0000000000000000  000003a0
       0000000000000060  0000000000000018   I      17    14     8
  [16] .shstrtab         STRTAB           0000000000000000  00000400
       000000000000009f  0000000000000000           0     0     1
  [17] .symtab           SYMTAB           0000000000000000  00000180
       0000000000000150  0000000000000018          18    12     8
  [18] .strtab           STRTAB           0000000000000000  000002d0
       000000000000000d  0000000000000000           0     0     1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), l (large)
  I (info), L (link order), G (group), T (TLS), E (exclude), x (unknown)
  O (extra OS processing required) o (OS specific), p (processor specific)
