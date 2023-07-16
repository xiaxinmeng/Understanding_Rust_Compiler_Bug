console
$ (lldb) x/4xg 0x83bc830+0xd839f28
0x15bf6758: 0xf7c45042f7c45042 0xf7c45042f7c45042
0x15bf6768: 0xf7c45042f7c45042 0xf7c45042f7c45042

$ (lldb) memory find -c2 -e 0xe0b62e2929aba83c -- 0x00400000 0x20000000
data found at location: 0x12b6de68
0x12b6de68: 3c a8 ab 29 29 2e b6 e0 26 49 0b ba d9 dc 71 8c
0x12b6de78: 6f 1b 8e 28 10 54 8e af 4b a2 b1 32 14 e9 71 db
no more matches within the range.
