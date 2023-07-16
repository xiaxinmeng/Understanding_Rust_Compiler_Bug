
thread #2, stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
    frame #0: 0x0000000100196852 nap`std::sys_common::thread_info::set::h958c88701b41d520 + 114
nap`std::sys_common::thread_info::set::h958c88701b41d520:
->  0x100196852 <+114>: vmovaps (%rax), %ymm0
    0x100196856 <+118>: vmovaps 0x60302(%rip), %ymm1
    0x10019685e <+126>: vmovaps %ymm1, (%rax)
    0x100196862 <+130>: movq   0x20(%rax), %rax
