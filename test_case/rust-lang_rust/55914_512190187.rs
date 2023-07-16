asm
example::foo2:
  sub rsp, 104
  mov dword ptr [rsp], edi
  mov dword ptr [rsp + 4], esi
  mov rax, rsp
  mov qword ptr [rsp + 8], rax
  lea rax, [rsp + 4]
  mov qword ptr [rsp + 16], rax
  cmp edi, esi
  jne .LBB8_1
  mov al, 1
  add rsp, 104
  ret
