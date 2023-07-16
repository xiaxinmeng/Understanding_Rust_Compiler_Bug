asm
0000000000000000 <_ZN5test21f17h7c517ed04f8b4e1bE>:
   0:   48 83 ec 18             sub    rsp,0x18
   4:   4c 8b 07                mov    r8,QWORD PTR [rdi]
   7:   48 8b 57 10             mov    rdx,QWORD PTR [rdi+0x10]
   b:   31 f6                   xor    esi,esi
   d:   31 c0                   xor    eax,eax
   f:   48 39 d6                cmp    rsi,rdx
  12:   72 18                   jb     2c <_ZN5test21f17h7c517ed04f8b4e1bE+0x2c>
  14:   eb 1f                   jmp    35 <_ZN5test21f17h7c517ed04f8b4e1bE+0x35>
  16:   66 2e 0f 1f 84 00 00    nop    WORD PTR cs:[rax+rax*1+0x0]
  1d:   00 00 00
  20:   49 03 04 f0             add    rax,QWORD PTR [r8+rsi*8]
  24:   48 89 fe                mov    rsi,rdi
  27:   48 39 d6                cmp    rsi,rdx
  2a:   73 09                   jae    35 <_ZN5test21f17h7c517ed04f8b4e1bE+0x35>
  2c:   48 89 f7                mov    rdi,rsi
  2f:   48 83 c7 01             add    rdi,0x1
  33:   73 0b                   jae    40 <_ZN5test21f17h7c517ed04f8b4e1bE+0x40>
  35:   48 89 f7                mov    rdi,rsi
  38:   31 c9                   xor    ecx,ecx
  3a:   31 f6                   xor    esi,esi
  3c:   eb 10                   jmp    4e <_ZN5test21f17h7c517ed04f8b4e1bE+0x4e>
  3e:   66 90                   xchg   ax,ax
  40:   48 c7 44 24 08 01 00    mov    QWORD PTR [rsp+0x8],0x1
  47:   00 00
  49:   b9 01 00 00 00          mov    ecx,0x1
  4e:   48 89 74 cc 08          mov    QWORD PTR [rsp+rcx*8+0x8],rsi
  53:   48 83 7c 24 08 01       cmp    QWORD PTR [rsp+0x8],0x1
  59:   75 18                   jne    73 <_ZN5test21f17h7c517ed04f8b4e1bE+0x73>
  5b:   48 8b 74 24 10          mov    rsi,QWORD PTR [rsp+0x10]
  60:   48 39 f2                cmp    rdx,rsi
  63:   77 bb                   ja     20 <_ZN5test21f17h7c517ed04f8b4e1bE+0x20>
  65:   48 8d 3d 00 00 00 00    lea    rdi,[rip+0x0]        # 6c <_ZN5test21f17h7c517ed04f8b4e1bE+0x6c>
  6c:   e8 00 00 00 00          call   71 <_ZN5test21f17h7c517ed04f8b4e1bE+0x71>
  71:   0f 0b                   ud2
  73:   48 83 c4 18             add    rsp,0x18
  77:   c3                      ret
