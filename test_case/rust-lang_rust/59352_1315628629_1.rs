
example::num_to_digit:
  lea ecx, [rdi - 48]
  xor eax, eax
  cmp ecx, 8
  cmovb eax, ecx
  ret
