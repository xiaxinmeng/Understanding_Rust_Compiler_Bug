`asm
example::test:
  push rbp
  mov rbp, rsp
  sub rsp, 72
  mov dword ptr [rbp - 8], 0
  mov qword ptr [rbp - 16], 0
  vmovdqu32 zmm0, zmmword ptr [rdx]
  vpaddd zmm0, zmm0, zmmword ptr [rsi]
  vmovdqu32 zmmword ptr [rbp - 72], zmm0
  mov eax, dword ptr [rdx + 64]
  add eax, dword ptr [rsi + 64]
  mov dword ptr [rbp - 8], eax
  mov dword ptr [rdi + 64], eax
  vmovdqu ymm0, ymmword ptr [rbp - 72]
  vmovdqu ymm1, ymmword ptr [rbp - 40]
  vmovdqu ymmword ptr [rdi + 32], ymm1
  vmovdqu ymmword ptr [rdi], ymm0
  mov rax, rdi
  add rsp, 72
  pop rbp
  ret
