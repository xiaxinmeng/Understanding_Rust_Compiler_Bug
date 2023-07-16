
(lldb) disas
diceware-c7f6b180dd3b52c6`std::panicking::panicking::h01f4a398b1d2259a:
    0x1000cc990 <+0>:  pushq  %rbp
    0x1000cc991 <+1>:  movq   %rsp, %rbp
    0x1000cc994 <+4>:  subq   $0x10, %rsp
    0x1000cc998 <+8>:  leaq   0xebc09(%rip), %rdi       ; std::panicking::update_panic_count::PANIC_COUNT::__getit::__KEY::hdcfcbe1f636ae13f
    0x1000cc99f <+15>: callq  *(%rdi)
    0x1000cc9a1 <+17>: cmpq   $0x1, (%rax)
    0x1000cc9a5 <+21>: jne    0x1000cc9b6               ; <+38>
    0x1000cc9a7 <+23>: leaq   0xebbfa(%rip), %rdi       ; std::panicking::update_panic_count::PANIC_COUNT::__getit::__KEY::hdcfcbe1f636ae13f
    0x1000cc9ae <+30>: callq  *(%rdi)
    0x1000cc9b0 <+32>: movq   0x8(%rax), %rcx
    0x1000cc9b4 <+36>: jmp    0x1000cc9d7               ; <+71>
    0x1000cc9b6 <+38>: movl   $0x1, %eax
    0x1000cc9bb <+43>: movd   %rax, %xmm0
    0x1000cc9c0 <+48>: movdqa %xmm0, -0x10(%rbp)
    0x1000cc9c5 <+53>: leaq   0xebbdc(%rip), %rdi       ; std::panicking::update_panic_count::PANIC_COUNT::__getit::__KEY::hdcfcbe1f636ae13f
    0x1000cc9cc <+60>: callq  *(%rdi)
    0x1000cc9ce <+62>: movaps -0x10(%rbp), %xmm0
->  0x1000cc9d2 <+66>: movaps %xmm0, (%rax)
    0x1000cc9d5 <+69>: xorl   %ecx, %ecx
    0x1000cc9d7 <+71>: leaq   0xebbca(%rip), %rdi       ; std::panicking::update_panic_count::PANIC_COUNT::__getit::__KEY::hdcfcbe1f636ae13f
    0x1000cc9de <+78>: callq  *(%rdi)
    0x1000cc9e0 <+80>: movq   %rcx, 0x8(%rax)
    0x1000cc9e4 <+84>: testq  %rcx, %rcx
    0x1000cc9e7 <+87>: setne  %al
    0x1000cc9ea <+90>: addq   $0x10, %rsp
    0x1000cc9ee <+94>: popq   %rbp
    0x1000cc9ef <+95>: retq   

(lldb) reg read
General Purpose Registers:
       rax = 0x0000000100500358
       rbx = 0x000000010180d008
       rcx = 0x0000000000000000
       rdx = 0x0000000000000000
       rdi = 0x00000001001b85a8  diceware-c7f6b180dd3b52c6`std::panicking::update_panic_count::PANIC_COUNT::__getit::__KEY::hdcfcbe1f636ae13f
       rsi = 0x0000000000000103
       rbp = 0x00007000006068e0
       rsp = 0x00007000006068d0
        r8 = 0x0000000101217358
        r9 = 0x0000000000a45e09
       r10 = 0x0000000101217360
       r11 = 0xffffffff00000000
       r12 = 0x0000000000000001
       r13 = 0x0000000000001003
       r14 = 0x0000000000000000
       r15 = 0x0000000101217380
       rip = 0x00000001000cc9d2  diceware-c7f6b180dd3b52c6`std::panicking::panicking::h01f4a398b1d2259a + 66
    rflags = 0x0000000000010202
        cs = 0x000000000000002b
        fs = 0x0000000000000000
        gs = 0x0000000000000000
