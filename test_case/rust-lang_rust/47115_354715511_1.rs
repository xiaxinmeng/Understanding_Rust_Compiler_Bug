assembly
.LBB5_18:
  test rsi, rsi
  je .LBB5_20
  add rdx, -4
  movzx r10d, byte ptr [rsi]
  movzx eax, byte ptr [rsi + 1]
  movzx ebx, byte ptr [rsi + 2]
  lea rsi, [rsi + 4]
  imul r13d, ebx, 19595
  imul eax, eax, 38470
  imul ebx, r10d, 7471
  add ebx, eax
  add ebx, r13d
  shr ebx, 16
  mov byte ptr [rcx], bl
  mov byte ptr [rcx + 1], bl
  mov byte ptr [rcx + 2], bl
  mov byte ptr [rcx + 3], 0
  lea rcx, [rcx + 4]
  cmp rdx, 4
  jae .LBB5_18
