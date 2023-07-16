asm
example::swap_i48:
  mov eax, dword ptr [rdi]
  movzx ecx, word ptr [rdi + 4]
  movzx edx, word ptr [rsi + 4]
  mov word ptr [rdi + 4], dx
  mov edx, dword ptr [rsi]
  mov dword ptr [rdi], edx
  mov word ptr [rsi + 4], cx
  mov dword ptr [rsi], eax
  ret
