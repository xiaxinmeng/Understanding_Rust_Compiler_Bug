
0000000000001000 <_start>:
    1000:	53                   	push   %rbx
    1001:	48 89 fb             	mov    %rdi,%rbx
    1004:	ff 15 ee 2f 00 00    	call   *0x2fee(%rip)        # 3ff8 <_GLOBAL_OFFSET_TABLE_+0x18>
    100a:	c6 43 03 61          	movb   $0x61,0x3(%rbx)
    100e:	48 8d 05 eb 0f 00 00 	lea    0xfeb(%rip),%rax        # 2000 <core::slice::raw::debug_check_data_len+0xfe9>
    1015:	5b                   	pop    %rbx
    1016:	c3                   	ret    

0000000000001017 <core::slice::raw::debug_check_data_len>:
    1017:	c3                   	ret    
