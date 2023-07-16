
   0x0000000000403420 <+0>:     sub    $0x18,%rsp
   0x0000000000403424 <+4>:     int3   
   0x0000000000403425 <+5>:     mov    %esp,%edi -- looks suspicious, given that %rdi is truncated
   0x0000000000403427 <+7>:     callq  *0x3c7cdb(%rip)        # 0x7cb108
=> 0x000000000040342d <+13>:    mov    %rsp,%rdi
   0x0000000000403430 <+16>:    callq  0x4036b0 <core::ptr::real_drop_in_place>
   0x0000000000403435 <+21>:    add    $0x18,%rsp
   0x0000000000403439 <+25>:    retq   

Dump of assembler code for function tarpotest::b::{{closure}}:
   0x0000000000403860 <+0>:     push   %rax
   0x0000000000403861 <+1>:     mov    %rdi,(%rsp)
   0x0000000000403865 <+5>:     callq  0x403420 <tarpotest::b>
   0x000000000040386a <+10>:    callq  0x403880 <test::assert_test_result>
   0x000000000040386f <+15>:    pop    %rax
   0x0000000000403870 <+16>:    retq   
