asm
example::foo2:
  sub rsp, 56
  mov dword ptr [rsp], edi
  mov dword ptr [rsp + 4], esi
  cmp edi, esi
  jne .LBB4_1
  mov al, 1
  add rsp, 56
  ret
.LBB4_1:
// panic
