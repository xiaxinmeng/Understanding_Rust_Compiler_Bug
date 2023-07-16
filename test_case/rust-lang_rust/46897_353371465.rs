asm
0000000000000000 <_ZN5test21f17h7c517ed04f8b4e1bE>:
   0:   48 8b 57 10             mov    rdx,QWORD PTR [rdi+0x10]
   4:   48 85 d2                test   rdx,rdx
   7:   74 1b                   je     24 <_ZN5test21f17h7c517ed04f8b4e1bE+0x24>
   9:   48 8b 0f                mov    rcx,QWORD PTR [rdi]
   c:   31 c0                   xor    eax,eax
   e:   31 f6                   xor    esi,esi
  10:   48 39 f2                cmp    rdx,rsi
  13:   76 12                   jbe    27 <_ZN5test21f17h7c517ed04f8b4e1bE+0x27>
  15:   48 03 04 f1             add    rax,QWORD PTR [rcx+rsi*8]
  19:   48 83 c6 01             add    rsi,0x1
  1d:   48 39 d6                cmp    rsi,rdx
  20:   72 ee                   jb     10 <_ZN5test21f17h7c517ed04f8b4e1bE+0x10>
  22:   eb 02                   jmp    26 <_ZN5test21f17h7c517ed04f8b4e1bE+0x26>
  24:   31 c0                   xor    eax,eax
  26:   c3                      ret
  27:   50                      push   rax
  28:   48 8d 3d 00 00 00 00    lea    rdi,[rip+0x0]        # 2f <_ZN5test21f17h7c517ed04f8b4e1bE+0x2f>
  2f:   e8 00 00 00 00          call   34 <_ZN5test21f17h7c517ed04f8b4e1bE+0x34>
  34:   0f 0b                   ud2
