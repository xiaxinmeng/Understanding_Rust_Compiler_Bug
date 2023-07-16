asm
check_stage1::foo:
 xor     eax, eax
 test    rdi, rdi
 je      .LBB0_5
 cmp     rsi, 2
 jne     .LBB0_5
 lea     rax, [rip, +, .Lanon.7bddf4f09674752ba4bdf737f126bcf4.0]
 cmp     rdi, rax
 je      .LBB0_3
 movzx   eax, word, ptr, [rdi]
 cmp     eax, 28531
 sete    al
.LBB0_5:
 ret
.LBB0_3:
 mov     al, 1
 ret
