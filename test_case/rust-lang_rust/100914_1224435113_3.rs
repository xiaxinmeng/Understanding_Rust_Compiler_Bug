
    8b74:       bf 04 00 00 00          mov    $0x4,%edi
    8b79:       be 04 00 00 00          mov    $0x4,%esi
    8b7e:       ff 15 dc 8e 04 00       call   *0x48edc(%rip)        # 51a60 <_GLOBAL_OFFSET_TABLE_+0x208>  (NB: __rust_alloc)
    8b84:       48 85 c0                test   %rax,%rax
    8b87:       0f 84 33 01 00 00       je     8cc0 <play::perform_double_free+0x160>
    8b8d:       c7 00 00 00 00 00       movl   $0x0,(%rax)
    8b93:       48 89 44 24 10          mov    %rax,0x10(%rsp)
    8b98:       48 c7 44 24 18 01 00    movq   $0x1,0x18(%rsp)
    8b9f:       00 00
    8ba1:       48 c7 44 24 20 01 00    movq   $0x1,0x20(%rsp)
    8ba8:       00 00
    8baa:       bf 04 00 00 00          mov    $0x4,%edi
    8baf:       be 04 00 00 00          mov    $0x4,%esi
    8bb4:       ff 15 a6 8e 04 00       call   *0x48ea6(%rip)        # 51a60 <_GLOBAL_OFFSET_TABLE_+0x208>  (NB: __rust_alloc)
    8bba:       48 85 c0                test   %rax,%rax
    8bbd:       0f 84 0f 01 00 00       je     8cd2 <play::perform_double_free+0x172>
    8bc3:       c7 00 00 00 00 00       movl   $0x0,(%rax)
    8bc9:       48 89 04 24             mov    %rax,(%rsp)
    8bcd:       48 c7 44 24 08 01 00    movq   $0x1,0x8(%rsp)
    8bd4:       00 00
    8bd6:       48 c7 44 24 10 01 00    movq   $0x1,0x10(%rsp)
    8bdd:       00 00
    8bdf:       40 b5 01                mov    $0x1,%bpl
    8be2:       48 89 e7                mov    %rsp,%rdi
    8be5:       e8 26 01 00 00          call   8d10 <play::verbose_drop>
    8bea:       48 8b 44 24 20          mov    0x20(%rsp),%rax
    8bef:       48 89 44 24 10          mov    %rax,0x10(%rsp)
    8bf4:       0f 10 44 24 10          movups 0x10(%rsp),%xmm0
    8bf9:       0f 29 04 24             movaps %xmm0,(%rsp)
    8bfd:       31 ed                   xor    %ebp,%ebp
    8bff:       48 89 e7                mov    %rsp,%rdi
    8c02:       e8 09 01 00 00          call   8d10 <play::verbose_drop>
