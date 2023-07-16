 asm
   6:   48 85 d2                test   %rdx,%rdx
   9:   0f 84 60 02 00 00       je     26f <str::from_utf8::hdcda68873f01cd68dYR+0x26f>
   f:   4c 8d 5a f0             lea    0xfffffffffffffff0(%rdx),%r11
  13:   31 c9                   xor    %ecx,%ecx
  15:   4c 8d 0d 00 00 00 00    lea    0(%rip),%r9        # 1c <str::from_utf8::hdcda68873f01cd68dYR+0x1c>
  1c:   41 b8 01 c0 00 00       mov    $0xc001,%r8d
  22:   49 ba 80 80 80 80 80    mov    $0x8080808080808080,%r10
  29:   80 80 80 
  2c:   8d 74 26 00             lea    0x0(%rsi),%esi
  30:   44 0f b6 3c 0e          movzbl (%rsi,%rcx,1),%r15d
  35:   45 84 ff                test   %r15b,%r15b
  38:   78 16                   js     50 <str::from_utf8::hdcda68873f01cd68dYR+0x50>
