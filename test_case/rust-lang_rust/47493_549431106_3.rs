
$ readelf -r foo.foo.7rcbfp3g-cgu.3.rcgu.o

Relocation section '.rela.text._ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E' at offset 0x2c8 contains 4 entries:
  Offset          Info           Type           Sym. Value    Sym. Name + Addend
000000000020  000800000004 R_X86_64_PLT32    0000000000000000 _Unwind_Resume - 4
000000000047  000500000002 R_X86_64_PC32     0000000000000000 .rodata..L__unnamed_1 - 4
00000000004e  000600000002 R_X86_64_PC32     0000000000000000 .data.rel.ro..L__unnam - 4
000000000055  000b00000009 R_X86_64_GOTPCREL 0000000000000000 _ZN4core6result13unwra - 4

Relocation section '.rela.data.rel.ro..L__unnamed_2' at offset 0x328 contains 2 entries:
  Offset          Info           Type           Sym. Value    Sym. Name + Addend
000000000000  000a00000001 R_X86_64_64       0000000000000000 _ZN4core3ptr18real_dro + 0
000000000018  000900000001 R_X86_64_64       0000000000000000 _ZN4core3fmt3num52_$LT + 0

Relocation section '.rela.data.DW.ref.rust_eh_personality' at offset 0x358 contains 1 entry:
  Offset          Info           Type           Sym. Value    Sym. Name + Addend
000000000000  000d00000001 R_X86_64_64       0000000000000000 rust_eh_personality + 0

Relocation section '.rela.eh_frame' at offset 0x370 contains 3 entries:
  Offset          Info           Type           Sym. Value    Sym. Name + Addend
000000000013  000700000002 R_X86_64_PC32     0000000000000000 DW.ref.rust_eh_persona + 0
000000000028  000300000002 R_X86_64_PC32     0000000000000000 .text._ZN4core6result1 + 0
000000000031  000400000002 R_X86_64_PC32     0000000000000000 .gcc_except_table + 0
