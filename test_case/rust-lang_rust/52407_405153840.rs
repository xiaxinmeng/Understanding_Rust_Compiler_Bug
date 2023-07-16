
$sudo dtruss /target/release/count
[snip]

write(0x1, "1\n\0", 0x2)		 = 2 0
line_2
2
read(0x0, "line_2\n\0", 0x2000)		 = 7 0
write(0x1, "2\n\0", 0x2)		 = 2 0
line_three
3
read(0x0, "line_three\n\0", 0x2000)		 = 11 0
write(0x1, "3\n\0", 0x2)		 = 2 0
3D
3
read(0x0, "t\b\0", 0x2000)		 = 0 0
write(0x1, "3\n\0", 0x2)		 = 2 0
write(0x1, "3\n\0", 0x2)		 = 2 0
sigaltstack(0x7FFEE1D88A50, 0x0, 0x0)		 = 0 0
munmap(0x10DF25000, 0x20000)		 = 0 0
