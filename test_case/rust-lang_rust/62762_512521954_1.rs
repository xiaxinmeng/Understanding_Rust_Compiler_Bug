
* thread #1, stop reason = Exception 0xc0000005 encountered at address 0x13fdfcf87
    frame #0: 0x000000013fdfcf87 rand_test.exe
->  0x13fdfcf87: vmovaps 0xa0(%rcx), %ymm0
    0x13fdfcf8f: movl   $0x1, %edx
    0x13fdfcf94: vmovq  %rdx, %xmm1
    0x13fdfcf99: vmovaps %xmm1, %xmm2
