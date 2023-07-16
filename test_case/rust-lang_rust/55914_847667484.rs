asm
example::foo1:
  cmp edi, esi
  jne .LBB3_1
  mov al, 1
  ret
.LBB3_1:
// panic
