rust
asm!(
  "movabsq $3f, %rax",
  "3:",
  "jmp *%rax",
  options(att_syntax),
);
