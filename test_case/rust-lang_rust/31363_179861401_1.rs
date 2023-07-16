 asm
00000000000f95e0 <str::from_utf8::hdcda68873f01cd68dYR>:
   f95e0:       55                      push   %rbp
   f95e1:       41 57                   push   %r15
   f95e3:       41 56                   push   %r14
   f95e5:       53                      push   %rbx
   f95e6:       48 85 d2                test   %rdx,%rdx
   f95e9:       0f 84 60 02 00 00       je     f984f <str::from_utf8::hdcda68873f01cd68dYR+0x26f>
   f95ef:       4c 8d 5a f0             lea    0xfffffffffffffff0(%rdx),%r11
   f95f3:       31 c9                   xor    %ecx,%ecx
   f95f5:       4c 8d 0d 11 c3 12 00    lea    1229585(%rip),%r9        # 22590d <str::UTF8_CHAR_WIDTH::he77aff40c91d8da6qNS>
   f95fc:       41 b8 01 c0 00 00       mov    $0xc001,%r8d
   f9602:       49 ba 80 80 80 80 80    mov    $0x8080808080808080,%r10
   f9609:       80 80 80 
   f960c:       8d 74 26 00             lea    0x0(%rsi),%esi
   f9610:       44 0f b6 3c 0e          movzbl (%rsi,%rcx,1),%r15d
   f9615:       45 84 ff                test   %r15b,%r15b
   f9618:       78 16                   js     f9630 <str::from_utf8::hdcda68873f01cd68dYR+0x50>
   f961a:       8d 04 31                lea    (%rcx,%rsi,1),%eax
   f961d:       a8 07                   test   $0x7,%al
   f961f:       74 5f                   je     f9680 <str::from_utf8::hdcda68873f01cd68dYR+0xa0>
