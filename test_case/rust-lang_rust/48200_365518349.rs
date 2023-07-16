asm
example::foo:
  push rbp
  mov rbp, rsp
  xor eax, eax
  cmp qword ptr [rsi], 1
  jne .LBB0_4
  cmp qword ptr [rdi], 1
  jne .LBB0_3
  mov rax, qword ptr [rdi + 8]
  add rax, qword ptr [rsi + 8]
  pop rbp
  ret
.LBB0_3:
  xor eax, eax
.LBB0_4:
  pop rbp
  ret

example::bar:
  push rbp
  mov rbp, rsp
  xor eax, eax
  cmp qword ptr [rdi], 1
  jne .LBB1_3
  xor eax, eax
  cmp qword ptr [rsi], 1
  jne .LBB1_3
  mov rax, qword ptr [rsi + 8]
  add rax, qword ptr [rdi + 8]
.LBB1_3:
  pop rbp
  ret
