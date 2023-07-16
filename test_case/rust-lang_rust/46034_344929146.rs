
$ readelf -S -W 1.13.0
There are 42 section headers, starting at offset 0x9d838:

Section Headers:
  [Nr] Name              Type            Address          Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            0000000000000000 000000 000000 00      0   0  0
  [ 1] .interp           PROGBITS        0000000000000270 000270 00001c 00   A  0   0  1
  [ 2] .note.ABI-tag     NOTE            000000000000028c 00028c 000020 00   A  0   0  4
  [ 3] .note.gnu.build-id NOTE            00000000000002ac 0002ac 000024 00   A  0   0  4
  [ 4] .gnu.hash         GNU_HASH        00000000000002d0 0002d0 0000b4 00   A  5   0  8
  [ 5] .dynsym           DYNSYM          0000000000000388 000388 0009c0 18   A  6   3  8
  [ 6] .dynstr           STRTAB          0000000000000d48 000d48 00065a 00   A  0   0  1
  [ 7] .gnu.version      VERSYM          00000000000013a2 0013a2 0000d0 02   A  5   0  2
  [ 8] .gnu.version_r    VERNEED         0000000000001478 001478 000110 00   A  6   4  8
  [ 9] .rela.dyn         RELA            0000000000001588 001588 003558 18   A  5   0  8
  [10] .rela.plt         RELA            0000000000004ae0 004ae0 000768 18  AI  5  27  8
  [11] .init             PROGBITS        0000000000005248 005248 00001a 00  AX  0   0  4
  [12] .plt              PROGBITS        0000000000005270 005270 000500 10  AX  0   0 16
  [13] .plt.got          PROGBITS        0000000000005770 005770 000018 00  AX  0   0  8
  [14] .text             PROGBITS        0000000000005790 005790 03fc69 00  AX  0   0 16
  [15] .fini             PROGBITS        00000000000453fc 0453fc 000009 00  AX  0   0  4
  [16] .rodata           PROGBITS        0000000000045440 045440 005031 00   A  0   0 64
  [17] .eh_frame_hdr     PROGBITS        000000000004a474 04a474 00148c 00   A  0   0  4
  [18] .eh_frame         PROGBITS        000000000004b900 04b900 006254 00   A  0   0  8
  [19] .gcc_except_table PROGBITS        0000000000051b54 051b54 002578 00   A  0   0  4
  [20] .tdata            PROGBITS        0000000000254d90 054d90 0000f0 00 WAT  0   0 16
  [21] .init_array       INIT_ARRAY      0000000000254e80 054e80 000010 00  WA  0   0  8
  [22] .fini_array       FINI_ARRAY      0000000000254e90 054e90 000008 00  WA  0   0  8
  [23] .jcr              PROGBITS        0000000000254e98 054e98 000008 00  WA  0   0  8
  [24] .data.rel.ro      PROGBITS        0000000000254ea0 054ea0 002ee8 00  WA  0   0 32
  [25] .dynamic          DYNAMIC         0000000000257d88 057d88 000220 10  WA  6   0  8
  [26] .got              PROGBITS        0000000000257fa8 057fa8 000040 08  WA  0   0  8
  [27] .got.plt          PROGBITS        0000000000258000 058000 000290 08  WA  0   0  8
  [28] .data             PROGBITS        00000000002582a0 0582a0 0001e1 00  WA  0   0 32
  [29] .bss              NOBITS          00000000002584a0 058481 000fd8 00  WA  0   0 32
  [30] .comment          PROGBITS        0000000000000000 058481 000079 01  MS  0   0  1
  [31] .debug_aranges    PROGBITS        0000000000000000 0584fa 0023a0 00      0   0  1
  [32] .debug_info       PROGBITS        0000000000000000 05a89a 00b63f 00      0   0  1
  [33] .debug_abbrev     PROGBITS        0000000000000000 065ed9 00186a 00      0   0  1
  [34] .debug_line       PROGBITS        0000000000000000 067743 002b12 00      0   0  1
  [35] .debug_frame      PROGBITS        0000000000000000 06a258 006a60 00      0   0  8
  [36] .debug_str        PROGBITS        0000000000000000 070cb8 007aee 01  MS  0   0  1
  [37] .debug_loc        PROGBITS        0000000000000000 0787a6 010c95 00      0   0  1
  [38] .debug_ranges     PROGBITS        0000000000000000 08943b 002220 00      0   0  1
  [39] .shstrtab         STRTAB          0000000000000000 09d69d 000198 00      0   0  1
  [40] .symtab           SYMTAB          0000000000000000 08b660 008a30 18     41 1199  8
  [41] .strtab           STRTAB          0000000000000000 094090 00960d 00      0   0  1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), l (large)
  I (info), L (link order), G (group), T (TLS), E (exclude), x (unknown)
  O (extra OS processing required) o (OS specific), p (processor specific)
