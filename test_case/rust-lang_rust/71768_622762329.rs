
0000000000005f10 <inc::inc_opt>:
    5f10:       sub    $0x10,%rsp
    5f14:       add    $0x2a,%rdi
    5f18:       mov    %rdi,0x8(%rsp)
    5f1d:       mov    0x8(%rsp),%rax
    5f22:       mov    %rax,(%rsp)
    5f26:       mov    (%rsp),%rax
    5f2a:       add    $0x10,%rsp
    5f2e:       retq   
    5f2f:       nop
