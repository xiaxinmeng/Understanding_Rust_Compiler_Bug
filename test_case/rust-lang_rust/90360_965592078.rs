asm
  0000000140203790: 48 8B 01           mov         rax,qword ptr [rcx]
  0000000140203793: 48 3B 04 11        cmp         rax,qword ptr [rcx+rdx]
  0000000140203797: 75 5B              jne         00000001402037F4
  0000000140203799: 48 8B 41 08        mov         rax,qword ptr [rcx+8]
  000000014020379D: 48 3B 44 11 08     cmp         rax,qword ptr [rcx+rdx+8]
  00000001402037A2: 75 4C              jne         00000001402037F0
  00000001402037A4: 48 8B 41 10        mov         rax,qword ptr [rcx+10h]
  00000001402037A8: 48 3B 44 11 10     cmp         rax,qword ptr [rcx+rdx+10h]
  00000001402037AD: 75 3D              jne         00000001402037EC
  00000001402037AF: 48 8B 41 18        mov         rax,qword ptr [rcx+18h]
  00000001402037B3: 48 3B 44 11 18     cmp         rax,qword ptr [rcx+rdx+18h]
  00000001402037B8: 75 2E              jne         00000001402037E8
  00000001402037BA: 48 83 C1 20        add         rcx,20h
  00000001402037BE: 49 FF C9           dec         r9
  00000001402037C1: 75 CD              jne         0000000140203790
