asm
example::num_to_digit_fast:
  add edi, -48
  xor eax, eax
  cmp edi, 8
  cmovb eax, edi
  ret

example::num_to_digit_fast2:
  add edi, -48
  xor eax, eax
  cmp edi, 7
  cmovbe eax, edi
  ret
