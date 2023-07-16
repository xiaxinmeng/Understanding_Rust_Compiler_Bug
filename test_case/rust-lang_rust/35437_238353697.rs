
C:\msys64\home\Peter\test> dumpbin /symbols libweak.rlib
Microsoft (R) COFF/PE Dumper Version 14.00.24210.0
Copyright (C) Microsoft Corporation.  All rights reserved.


Dump of file libweak.rlib

File Type: LIBRARY

COFF SYMBOL TABLE
000 00000000 SECT1  notype       Static       | .text
    Section length    0, #relocs    0, #linenums    0, checksum        0
002 00000000 SECT2  notype       Static       | .data
    Section length    0, #relocs    0, #linenums    0, checksum        0
004 00000000 SECT3  notype       Static       | .bss
    Section length    0, #relocs    0, #linenums    0, checksum        0
006 00000000 SECT4  notype       Static       | .text
    Section length    1, #relocs    0, #linenums    0, checksum  26D930A, selection    1 (pick no duplicates)
008 00000000 SECT4  notype ()    External     | memfoo
009 00000000 SECT5  notype       Static       | .xdata
    Section length    8, #relocs    0, #linenums    0, checksum CCAA009E, selection    5 (pick associative Section 0x4)
00B 00000000 SECT6  notype       Static       | .pdata
    Section length    C, #relocs    3, #linenums    0, checksum CCAA009E, selection    5 (pick associative Section 0x4)

String Table Size = 0x0 bytes
libweak.rlib : fatal error LNK1106: invalid file or disk full: cannot seek to 0x31A1
