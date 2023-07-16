 asm
00000000000f8e50 <str::from_utf8::hf64d73b10248ef4bdYR>:
   f8e50:       55                      push   %rbp
   f8e51:       41 57                   push   %r15
   f8e53:       41 56                   push   %r14
   f8e55:       53                      push   %rbx
   f8e56:       48 85 d2                test   %rdx,%rdx
   f8e59:       0f 84 65 02 00 00       je     f90c4 <str::from_utf8::hf64d73b10248ef4bdYR+0x274>
   f8e5f:       4c 8d 5a f0             lea    0xfffffffffffffff0(%rdx),%r11
   f8e63:       31 c9                   xor    %ecx,%ecx
   f8e65:       4c 8d 0d 2d bb 12 00    lea    1227565(%rip),%r9        # 224999 <str::UTF8_CHAR_WIDTH::hb3baaadb65d72bbdqNS>
   f8e6c:       41 b8 01 c0 00 00       mov    $0xc001,%r8d
   f8e72:       49 ba 80 80 80 80 80    mov    $0x8080808080808080,%r10
   f8e79:       80 80 80 
   f8e7c:       90                      nop    
   f8e7d:       90                      nop    
   f8e7e:       90                      nop    
   f8e7f:       90                      nop    
   f8e80:       44 0f b6 3c 0e          movzbl (%rsi,%rcx,1),%r15d
   f8e85:       45 84 ff                test   %r15b,%r15b
   f8e88:       78 16                   js     f8ea0 <str::from_utf8::hf64d73b10248ef4bdYR+0x50>
   f8e8a:       8d 04 31                lea    (%rcx,%rsi,1),%eax
   f8e8d:       a8 07                   test   $0x7,%al
   f8e8f:       0f 84 1b 01 00 00       je     f8fb0 <str::from_utf8::hf64d73b10248ef4bdYR+0x160>
...
