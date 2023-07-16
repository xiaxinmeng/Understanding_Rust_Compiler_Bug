
(lldb) dis
text_util`std::panicking::panicking::hbf60d20c3ea01bc0:
    0x10005c880 <+0>:  pushq  %rbp
    0x10005c881 <+1>:  movq   %rsp, %rbp
    0x10005c884 <+4>:  subq   $0x10, %rsp
    0x10005c888 <+8>:  leaq   0x6a349(%rip), %rdi       ; std::panicking::update_panic_count::PANIC_COUNT::__getit::__KEY::hfd55c7d72b45f2cb
    0x10005c88f <+15>: callq  *(%rdi)
    0x10005c891 <+17>: cmpq   $0x1, (%rax)
    0x10005c895 <+21>: jne    0x10005c8a6               ; <+38> [inlined] core::ptr::swap_nonoverlapping_bytes::h5ceec518bedbc8b5 at ptr.rs:187
    0x10005c897 <+23>: leaq   0x6a33a(%rip), %rdi       ; std::panicking::update_panic_count::PANIC_COUNT::__getit::__KEY::hfd55c7d72b45f2cb
    0x10005c89e <+30>: callq  *(%rdi)
    0x10005c8a0 <+32>: movq   0x8(%rax), %rcx
    0x10005c8a4 <+36>: jmp    0x10005c8c7               ; <+71> [inlined] core::ptr::swap_nonoverlapping_bytes::h5ceec518bedbc8b5 at ptr.rs:187
    0x10005c8a6 <+38>: movl   $0x1, %eax
    0x10005c8ab <+43>: movd   %rax, %xmm0
    0x10005c8b0 <+48>: movdqa %xmm0, -0x10(%rbp)
    0x10005c8b5 <+53>: leaq   0x6a31c(%rip), %rdi       ; std::panicking::update_panic_count::PANIC_COUNT::__getit::__KEY::hfd55c7d72b45f2cb
    0x10005c8bc <+60>: callq  *(%rdi)
    0x10005c8be <+62>: movaps -0x10(%rbp), %xmm0
->  0x10005c8c2 <+66>: movaps %xmm0, (%rax)
    0x10005c8c5 <+69>: xorl   %ecx, %ecx
    0x10005c8c7 <+71>: leaq   0x6a30a(%rip), %rdi       ; std::panicking::update_panic_count::PANIC_COUNT::__getit::__KEY::hfd55c7d72b45f2cb
    0x10005c8ce <+78>: callq  *(%rdi)
    0x10005c8d0 <+80>: movq   %rcx, 0x8(%rax)
    0x10005c8d4 <+84>: testq  %rcx, %rcx
    0x10005c8d7 <+87>: setne  %al
    0x10005c8da <+90>: addq   $0x10, %rsp
    0x10005c8de <+94>: popq   %rbp
    0x10005c8df <+95>: retq
(lldb) x $rax
0x1003000c8: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
0x1003000d8: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
