
example::num_to_digit:
  lea ecx, [rdi - 48]
  mov edx, edi
  and edx, -8
  xor eax, eax
  cmp edx, 48
  cmove eax, ecx
  ret
