
$ readelf -S -W 1.14.0
There are 45 section headers, starting at offset 0x1f8318:

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
  [ 9] .rela.dyn         RELA            0000000000001588 001588 0035a0 18   A  5   0  8
  [10] .rela.plt         RELA            0000000000004b28 004b28 000768 18  AI  5  27  8
  [11] .init             PROGBITS        0000000000005290 005290 00001a 00  AX  0   0  4
  [12] .plt              PROGBITS        00000000000052b0 0052b0 000500 10  AX  0   0 16
  [13] .plt.got          PROGBITS        00000000000057b0 0057b0 000018 00  AX  0   0  8
  [14] .text             PROGBITS        00000000000057d0 0057d0 041f29 00  AX  0   0 16
  [15] .fini             PROGBITS        00000000000476fc 0476fc 000009 00  AX  0   0  4
  [16] .rodata           PROGBITS        0000000000047740 047740 0052b1 00   A  0   0 64
  [17] .eh_frame_hdr     PROGBITS        000000000004c9f4 04c9f4 0014d4 00   A  0   0  4
  [18] .eh_frame         PROGBITS        000000000004dec8 04dec8 00645c 00   A  0   0  8
  [19] .gcc_except_table PROGBITS        0000000000054324 054324 002700 00   A  0   0  4
  [20] .tdata            PROGBITS        0000000000256d50 056d50 0000f0 00 WAT  0   0 16
  [21] .init_array       INIT_ARRAY      0000000000256e40 056e40 000010 00  WA  0   0  8
  [22] .fini_array       FINI_ARRAY      0000000000256e50 056e50 000008 00  WA  0   0  8
  [23] .jcr              PROGBITS        0000000000256e58 056e58 000008 00  WA  0   0  8
  [24] .data.rel.ro      PROGBITS        0000000000256e60 056e60 002f28 00  WA  0   0 32
  [25] .dynamic          DYNAMIC         0000000000259d88 059d88 000220 10  WA  6   0  8
  [26] .got              PROGBITS        0000000000259fa8 059fa8 000040 08  WA  0   0  8
  [27] .got.plt          PROGBITS        000000000025a000 05a000 000290 08  WA  0   0  8
  [28] .data             PROGBITS        000000000025a2a0 05a2a0 0001e1 00  WA  0   0 32
  [29] .bss              NOBITS          000000000025a4a0 05a481 000ff8 00  WA  0   0 32
  [30] .comment          PROGBITS        0000000000000000 05a481 000079 01  MS  0   0  1
  [31] .debug_aranges    PROGBITS        0000000000000000 05a4fa 0023a0 00      0   0  1
  [32] .debug_pubnames   PROGBITS        0000000000000000 05c89a 01a19a 00      0   0  1
  [33] .debug_info       PROGBITS        0000000000000000 076a34 0716cd 00      0   0  1
  [34] .debug_abbrev     PROGBITS        0000000000000000 0e8101 002763 00      0   0  1
  [35] .debug_line       PROGBITS        0000000000000000 0ea864 02c556 00      0   0  1
  [36] .debug_frame      PROGBITS        0000000000000000 116dc0 006a60 00      0   0  8
  [37] .debug_str        PROGBITS        0000000000000000 11d820 056459 01  MS  0   0  1
  [38] .debug_loc        PROGBITS        0000000000000000 173c79 010c95 00      0   0  1
  [39] .debug_macinfo    PROGBITS        0000000000000000 18490e 000008 00      0   0  1
  [40] .debug_pubtypes   PROGBITS        0000000000000000 184916 00ab8a 00      0   0  1
  [41] .debug_ranges     PROGBITS        0000000000000000 18f4a0 0567a0 00      0   0  1
  [42] .shstrtab         STRTAB          0000000000000000 1f814c 0001c7 00      0   0  1
  [43] .symtab           SYMTAB          0000000000000000 1e5c40 008da8 18     44 1238  8
  [44] .strtab           STRTAB          0000000000000000 1ee9e8 009764 00      0   0  1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), l (large)
  I (info), L (link order), G (group), T (TLS), E (exclude), x (unknown)
  O (extra OS processing required) o (OS specific), p (processor specific)
