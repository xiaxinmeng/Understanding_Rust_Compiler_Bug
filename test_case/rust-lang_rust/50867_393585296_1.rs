
    0x10005020c <+108>: jne    0x10005026e               ; <+206> [inlined] _$LT$core..cell..Cell$LT$T$GT$$GT$::get::h6870a5734ed219a9 at cell.rs:1013
    0x10005020e <+110>: leaq   0x5f513(%rip), %rdi       ; std::sys_common::thread_info::THREAD_INFO::__getit::__KEY::hc2d4dd6884ed9a0e
    0x100050215 <+117>: callq  *(%rdi)
->  0x100050217 <+119>: vmovaps (%rax), %ymm0
    0x10005021b <+123>: vmovaps 0x3bc5d(%rip), %ymm1
    0x100050223 <+131>: vmovaps %ymm1, (%rax)
    0x100050227 <+135>: movq   0x20(%rax), %rax
