
readelf -WS ./build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5ccd458f55feeb13.so
There are 32 section headers, starting at offset 0xde9e28:

Section Headers:
  [Nr] Name              Type            Address          Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            0000000000000000 000000 000000 00      0   0  0
  [ 1] .note.gnu.build-id NOTE            0000000000000270 000270 000024 00   A  0   0  4
  [ 2] .gnu.hash         GNU_HASH        0000000000000298 000298 004060 00   A  3   0  8
  [ 3] .dynsym           DYNSYM          00000000000042f8 0042f8 010260 18   A  4   1  8
  [ 4] .dynstr           STRTAB          0000000000014558 014558 039a6c 00   A  0   0  1
  [ 5] .gnu.version      VERSYM          000000000004dfc4 04dfc4 001588 02   A  3   0  2
  [ 6] .gnu.version_r    VERNEED         000000000004f550 04f550 000170 00   A  4   5  8
  [ 7] .rela.dyn         RELA            000000000004f6c0 04f6c0 010da0 18   A  3   0  8
  [ 8] .rela.plt         RELA            0000000000060460 060460 000090 18  AI  3  24  8
  [ 9] .init             PROGBITS        0000000000061000 061000 000017 00  AX  0   0  4
  [10] .plt              PROGBITS        0000000000061020 061020 000070 10  AX  0   0 16
  [11] .plt.got          PROGBITS        0000000000061090 061090 000008 08  AX  0   0  8
  [12] .text             PROGBITS        00000000000610a0 0610a0 0dc485 00  AX  0   0 16
  [13] .fini             PROGBITS        000000000013d528 13d528 000009 00  AX  0   0  4
  [14] .rodata           PROGBITS        000000000013e000 13e000 025ada 00   A  0   0 16
  [15] .llvmbc           PROGBITS        0000000000163ae0 163ae0 8a5740 00   A  0   0 16
  [16] .eh_frame_hdr     PROGBITS        0000000000a09220 a09220 009ac4 00   A  0   0  4
  [17] .eh_frame         PROGBITS        0000000000a12ce8 a12ce8 02b948 00   A  0   0  8
  [18] .gcc_except_table PROGBITS        0000000000a3e630 a3e630 002ddc 00   A  0   0  4
  [19] .tbss             NOBITS          0000000000a42978 a41978 0000c0 00 WAT  0   0  8
  [20] .init_array       INIT_ARRAY      0000000000a42978 a41978 000010 08  WA  0   0  8
  [21] .fini_array       FINI_ARRAY      0000000000a42988 a41988 000008 08  WA  0   0  8
  [22] .data.rel.ro      PROGBITS        0000000000a42990 a41990 00a518 00  WA  0   0  8
  [23] .dynamic          DYNAMIC         0000000000a4cea8 a4bea8 000230 10  WA  4   0  8
  [24] .got              PROGBITS        0000000000a4d0d8 a4c0d8 000f28 08  WA  0   0  8
  [25] .data             PROGBITS        0000000000a4e000 a4d000 000088 00  WA  0   0  8
  [26] .bss              NOBITS          0000000000a4e088 a4d088 0002d8 00  WA  0   0  8
  [27] .comment          PROGBITS        0000000000000000 a4d088 00001d 01  MS  0   0  1
  [28] .rustc            PROGBITS        0000000000000000 a4d0b0 2e9536 00      0   0 16
  [29] .symtab           SYMTAB          0000000000000000 d365e8 02ac90 18     30 4548  8
  [30] .strtab           STRTAB          0000000000000000 d61278 088a93 00      0   0  1
  [31] .shstrtab         STRTAB          0000000000000000 de9d0b 00011c 00      0   0  1
