asm
    32a6:       48 89 e7                mov    %rsp,%rdi
    32a9:       be 08 00 00 00          mov    $0x8,%esi
    32ae:       31 d2                   xor    %edx,%edx
    32b0:       ff 15 12 2d 02 00       callq  *0x22d12(%rip)        # 25fc8 <posix_memalign@GLIBC_2.2.5>
