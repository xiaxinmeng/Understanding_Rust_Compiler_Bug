
* thread #1, queue = 'com.apple.main-thread', stop reason = signal SIGSTOP
    frame #0: 0x0000000103f80550 bench-d5493d9f2fa74606`std::sys::unix::stack_overflow::imp::signal_handler::h373dcc530442aa18 (.llvm.16322965540172889217) + 112
bench-d5493d9f2fa74606`std::sys::unix::stack_overflow::imp::signal_handler::h373dcc530442aa18:
->  0x103f80550 <+112>: vmovdqa (%rax), %ymm0
    0x103f80554 <+116>: vmovaps 0x45c24(%rip), %ymm1
    0x103f8055c <+124>: vmovaps %ymm1, (%rax)
    0x103f80560 <+128>: movq   0x20(%rax), %rax
Target 0: (bench-d5493d9f2fa74606) stopped.
