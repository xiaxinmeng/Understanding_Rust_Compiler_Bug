
    6f30:       48 83 f8 06             cmp    $0x6,%rax
    6f34:       b9 07 00 00 00          mov    $0x7,%ecx
    6f39:       48 0f 4f c8             cmovg  %rax,%rcx
    6f3d:       f0 48 0f b1 4c 24 08    lock cmpxchg %rcx,0x8(%rsp)
    6f44:       75 ea                   jne    6f30 <_ZN3amx4main17h5557c2e4c983c622E+0x20>
