
.intel_syntax noprefix
   mov  edi, offset 2f         # fails
2:
   push  offset 2b           # also fails
