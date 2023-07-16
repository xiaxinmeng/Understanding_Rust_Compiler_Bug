
(gdb) print SPFlags
$22 = LLVMRustDISPFlags::SPFlagZero
(gdb) print &SPFlags
$23 = (LLVMRustDISPFlags *) 0xfff8000107f03afc
(gdb) x/2wx &SPFlags-1
0xfff8000107f03af8:     0x0000000c      0x00000000
