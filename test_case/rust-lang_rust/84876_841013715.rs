
C:\s>type t.c
int _tls_index;
__declspec(thread) int foo;

__declspec(dllexport)
int* GetFoo()
{
 return &foo;
}

cl /Ox /LD /Zi t.c /link /nod /noentry  /incremental:no
Microsoft (R) C/C++ Optimizing Compiler Version 19.28.29914 for x64

link /dump /disasm t.dll

GetFoo:
  0000000180001000: 8B 15 FA 1F 00 00  mov         edx,dword ptr [_tls_index]
  0000000180001006: 65 48 8B 0C 25 58  mov         rcx,qword ptr gs:[58h]
                    00 00 00
  000000018000100F: B8 FC 00 00 00     mov         eax,0FCh
  0000000180001014: 48 03 04 D1        add         rax,qword ptr [rcx+rdx*8]
  0000000180001018: C3                 ret
