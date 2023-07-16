asm
 mov     rax, rdi
 test    rdi, rdi // this instruction is redundant - should not be part of output
 ret
